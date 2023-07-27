use serde::Deserialize; // Импортируем трейт и одноименный дерайв-макрос из библиотеки
use std::fmt::{Display, Formatter, Result};

/// Перечисление всех возможных фруктов
#[derive(Deserialize)] // Реализуем трейт Deserialize с помощью макроса из библиотеки
pub enum FruitBox {
    Apple(Apple),
}

/// Красиво выводим перечисление фруктов
impl Display for FruitBox {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Apple(apple) => apple.fmt(f),
        }
    }
}

#[derive(Deserialize)] // Реализуем трейт Deserialize с помощью макроса из библиотеки
pub struct Apple;

/// Красиво выводим Яблоко
impl Display for Apple {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Apple")
    }
}

/// Перечисление всех возможных напитков
#[derive(Deserialize)] // Реализуем трейт Deserialize с помощью макроса из библиотеки
pub enum DrinkBox {
    Milk(Milk),
    CocaCola(CocaCola),
    Water(Water),
}

/// Красиво выводим перечисление напитков
impl Display for DrinkBox {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::CocaCola(coca_cola) => coca_cola.fmt(f),
            Self::Water(water) => water.fmt(f),
            Self::Milk(milk) => milk.fmt(f),
        }
    }
}


#[derive(Deserialize)] // Реализуем трейт Deserialize с помощью макроса из библиотеки
pub struct Milk {
    liters: f32,
    fat_percentage: f32,
}

/// Красиво выводим Молоко
impl Display for Milk {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} liters of {}% milk", self.liters, self.fat_percentage)
    }
}

#[derive(Deserialize)] // Реализуем трейт Deserialize с помощью макроса из библиотеки
pub struct CocaCola;

/// Красиво выводим Колу
impl Display for CocaCola {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "CocaCola")
    }
}

#[derive(Deserialize)] // Реализуем трейт Deserialize с помощью макроса из библиотеки
pub struct Water;

/// Красиво выводим Воду
impl Display for Water {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Water")
    }
}
