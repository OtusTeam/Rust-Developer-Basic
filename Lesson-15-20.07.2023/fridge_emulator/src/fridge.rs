use std::fmt::{Display, Formatter, Result};
use crate::food::Food;

pub struct Fridge {
    vec_of_food: Vec<Box<dyn Food>>,
}

impl Fridge {
    /// Создать новый пустой холодильник.
    pub fn new() -> Self {
        Self {
            vec_of_food: Vec::new(),
        }
    }

    /// Положить еду в холодильник.
    pub fn put(&mut self, food: impl Food + 'static) {
        // Оборачиваем еду в "коробку" и кладем последним элементом в вектор
        self.vec_of_food.push(Box::new(food))
    }
}

/// Реализуем красивую печать холодильника.
impl Display for Fridge {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // Печатаем описание каждый еды из холодильника
        for food in &self.vec_of_food {
            write!(f, "{}, ", food.description())?;
        }

        Ok(())
    }
}
