use serde::Deserialize; // Импортируем трейт и одноименный дерайв-макрос из библиотеки
use std::fmt::{Display, Formatter, Result};

/// Трейт, описывающий еду.
pub trait Food: Display {}

#[derive(Deserialize)] // Реализуем трейт Deserialize с помощью макроса из библиотеки
pub struct Milk {
    liters: f32,
    fat_percentage: f32,
}

/// Обозначаем что Молоко -- это еда.
impl Food for Milk {}

impl Display for Milk {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} liters of {}% milk", self.liters, self.fat_percentage)
    }
}

pub struct Eggs;

impl Food for Eggs {}

impl Display for Eggs {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Eggs")
    }
}

pub struct Apple;

impl Food for Apple {}

impl Display for Apple {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Apple")
    }
}
