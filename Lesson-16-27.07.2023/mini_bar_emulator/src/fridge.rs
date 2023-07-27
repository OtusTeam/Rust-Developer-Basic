use std::fmt::{Display, Formatter, Result};
use crate::food;

/// Трейт холодильника
pub trait Bar {
    /// Тип объектов, которые хранит холодильник
    type Item;

    /// Положить в холодильник
    fn put(&mut self, item: Self::Item);


    fn put_multiple(&mut self, items: Vec<Self::Item>) {
        for item in items {
            self.put(item)
        }
    }
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
impl Bar for DrinkBar {
    /// Он хранит напитки
    type Item = food::DrinkBox;

    /// Положить напиток в холодильник с напитками
    fn put(&mut self, drink: Self::Item) {
        self.drinks.push(drink)
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
impl Bar for FruitBar {
    /// Он хранит фрукты
    type Item = food::FruitBox;

    /// Положить фрукт в холодильник с фруктами
    fn put(&mut self, fruit: Self::Item) {
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
