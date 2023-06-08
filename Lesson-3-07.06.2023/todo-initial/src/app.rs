use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub struct App {
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            should_quit: false,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if matches!(key.code, KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL)) {
            self.should_quit = true;
        }
    }
}
