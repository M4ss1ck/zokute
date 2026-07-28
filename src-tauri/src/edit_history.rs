use crate::config::Profile;

pub struct EditHistory {
    undo: Vec<Profile>,
    redo: Vec<Profile>,
}

impl EditHistory {
    pub fn new() -> Self {
        EditHistory { undo: Vec::new(), redo: Vec::new() }
    }

    pub fn push_undo(&mut self, profile: Profile) {
        self.undo.push(profile);
        self.redo.clear();
    }

    pub fn undo(&mut self, current: Profile) -> Option<Profile> {
        let prev = self.undo.pop()?;
        self.redo.push(current);
        Some(prev)
    }

    pub fn redo(&mut self, current: Profile) -> Option<Profile> {
        let next = self.redo.pop()?;
        self.undo.push(current);
        Some(next)
    }

    pub fn can_undo(&self) -> bool {
        !self.undo.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo.is_empty()
    }

    pub fn clear(&mut self) {
        self.undo.clear();
        self.redo.clear();
    }
}
