use crate::config::Config;

pub struct EditHistory {
    undo: Vec<Config>,
    redo: Vec<Config>,
}

impl EditHistory {
    pub fn new() -> Self {
        EditHistory { undo: Vec::new(), redo: Vec::new() }
    }

    pub fn push_undo(&mut self, config: Config) {
        self.undo.push(config);
        self.redo.clear();
    }

    pub fn undo(&mut self, current: Config) -> Option<Config> {
        let prev = self.undo.pop()?;
        self.redo.push(current);
        Some(prev)
    }

    pub fn redo(&mut self, current: Config) -> Option<Config> {
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
