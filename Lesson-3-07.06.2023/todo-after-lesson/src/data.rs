use uuid::Uuid;

#[derive(Debug)]
pub struct TodoTask {
    pub id: Uuid,
    pub title: String,
    pub is_done: bool,
}

impl TodoTask {
    pub fn new(title: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            is_done: false,
        }
    }
}
