use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::widgets::ListState;

use crate::data::TodoTask;

pub struct App {
    pub task_list: Vec<TodoTask>,
    pub task_list_state: ListState,
    pub should_quit: bool,
}

impl App {
    pub fn new(task_list: Vec<TodoTask>) -> Self {
        App {
            task_list,
            task_list_state: ListState::default(),
            should_quit: false,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if matches!(
            key.code,
            KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL)
        ) {
            self.should_quit = true;
            return;
        }

        self.handle_list_key(key);
    }

    pub fn handle_list_key(&mut self, key: KeyEvent) {
        if self.task_list.is_empty() {
            return;
        }

        match key.code {
            KeyCode::Down => match self.task_list_state.selected() {
                Some(index) if index == self.task_list.len() - 1 => {
                    self.task_list_state.select(None);
                }
                Some(index) => {
                    self.task_list_state.select(Some(index + 1));
                }
                None => {
                    self.task_list_state.select(Some(0));
                }
            },
            KeyCode::Up => match self.task_list_state.selected() {
                Some(index) if index == 0 => {
                    self.task_list_state.select(None);
                }
                Some(index) => {
                    self.task_list_state.select(Some(index - 1));
                }
                None => {
                    self.task_list_state.select(Some(self.task_list.len() - 1));
                }
            },
            KeyCode::Char(' ') | KeyCode::Enter => {
                let Some(index) = self.task_list_state.selected() else {
                    return;
                };

                let Some(task) = self.task_list.get_mut(index) else {
                    return;
                };

                task.is_done = !task.is_done;
            }
            _ => {}
        }
    }
}
