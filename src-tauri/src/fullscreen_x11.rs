use std::time::{Duration, Instant};
use x11rb::connection::Connection;
use x11rb::protocol::xproto::{
    Atom, AtomEnum, ChangeWindowAttributesAux, ConnectionExt, EventMask, Window,
};
use x11rb::protocol::Event;
use x11rb::rust_connection::RustConnection;

/// EWMH fullscreen tracking for the active window. Reports state, never
/// dimensions, per FULL-002.
pub struct Session {
    conn: RustConnection,
    root: Window,
    active_window_atom: Atom,
    state_atom: Atom,
    fullscreen_atom: Atom,
    watched: Option<Window>,
}

impl Session {
    pub fn connect() -> Result<Self, Box<dyn std::error::Error>> {
        let (conn, screen) = x11rb::connect(None)?;
        let root = conn.setup().roots[screen].root;
        let atom = |name: &str| -> Result<Atom, Box<dyn std::error::Error>> {
            Ok(conn.intern_atom(false, name.as_bytes())?.reply()?.atom)
        };
        let active_window_atom = atom("_NET_ACTIVE_WINDOW")?;
        let state_atom = atom("_NET_WM_STATE")?;
        let fullscreen_atom = atom("_NET_WM_STATE_FULLSCREEN")?;
        conn.change_window_attributes(root, &property_changes())?.check()?;
        Ok(Session { conn, root, active_window_atom, state_atom, fullscreen_atom, watched: None })
    }

    fn active_window(&self) -> Option<Window> {
        let reply = self.conn
            .get_property(false, self.root, self.active_window_atom, AtomEnum::WINDOW, 0, 1)
            .ok()?
            .reply()
            .ok()?;
        let mut values = reply.value32()?;
        let id = values.next()?;
        (id != 0).then_some(id)
    }

    /// Follow whichever window holds focus so its own state changes reach us.
    pub fn watch_active(&mut self) {
        let window = self.active_window();
        if window == self.watched {
            return;
        }
        if let Some(window) = window {
            let _ = self.conn.change_window_attributes(window, &property_changes());
            let _ = self.conn.flush();
        }
        self.watched = window;
    }

    pub fn is_fullscreen(&self) -> bool {
        let Some(window) = self.watched else { return false };
        let Ok(cookie) = self.conn.get_property(false, window, self.state_atom, AtomEnum::ATOM, 0, 32) else {
            return false;
        };
        let Ok(reply) = cookie.reply() else { return false };
        reply.value32()
            .map(|mut atoms| atoms.any(|atom| atom == self.fullscreen_atom))
            .unwrap_or(false)
    }

    fn is_interesting(&self, event: &Event) -> bool {
        match event {
            Event::PropertyNotify(notify) => {
                notify.atom == self.active_window_atom || notify.atom == self.state_atom
            }
            _ => false,
        }
    }

    /// Blocks until the window manager reports something we care about. Costs
    /// nothing while the desktop is idle.
    pub fn wait_for_change(&self) -> Result<(), ()> {
        loop {
            let event = self.conn.wait_for_event().map_err(|_| ())?;
            if self.is_interesting(&event) {
                return Ok(());
            }
        }
    }

    /// Waits until `due`, returning early if the state changed meanwhile.
    pub fn wait_until(&self, due: Instant) {
        while Instant::now() < due {
            match self.conn.poll_for_event() {
                Ok(Some(event)) if self.is_interesting(&event) => return,
                Ok(_) => std::thread::sleep(Duration::from_millis(10)),
                Err(_) => return,
            }
        }
    }
}

fn property_changes() -> ChangeWindowAttributesAux {
    ChangeWindowAttributesAux::new().event_mask(EventMask::PROPERTY_CHANGE)
}
