use std::fmt::{Display, Formatter, Result};
use crate::food;

/// Трейт холодильника с дженериком
pub trait Bar<I> {
    /// Положить в холодильник
    fn put(&mut self, item: I);
}

/// Холодильник с напитками
pub struct DrinkBar {
    drinks: Vec<food::DrinkBox>,
}

impl DrinkBar {
    /// Создать новый [`DrinkBar`]
    pub fn new() -> Self {
        Self {
            drinks: Vec::new()
        }
    }
}

/// Определяем трейт холодильника для холодильника с напитками
impl Bar<food::DrinkBox> for DrinkBar {
    /// Положить напиток в холодильник с напитками
    fn put(&mut self, drink: food::DrinkBox) {
        self.drinks.push(drink)
    }
}

impl Bar<food::Apple> for DrinkBar {
    fn put(&mut self, _apple: food::Apple) {
        unimplemented!()
    }
}

/// Красиво выводим холодильник с напитками
impl Display for DrinkBar {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for drink in &self.drinks {
            write!(f, "{drink}, ")?;
        }

        Ok(())
    }
}

/// Холодильник с фруктами
pub struct FruitBar {
    fruits: Vec<food::FruitBox>,
}

impl FruitBar {
    /// Создать новый [`FruitBar`]
    pub fn new() -> Self {
        Self {
            fruits: Vec::new()
        }
    }
}

/// Определяем трейт холодильника для холодильника с фруктами
impl Bar<food::FruitBox> for FruitBar {
    /// Положить фрукт в холодильник с фруктами
    fn put(&mut self, fruit: food::FruitBox) {
        self.fruits.push(fruit)
    }
}

/// Красиво выводим холодильник с фруктами
impl Display for FruitBar {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for fruit in &self.fruits {
            write!(f, "{fruit}, ")?;
        }

        Ok(())
    }
}
