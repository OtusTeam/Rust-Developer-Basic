use serde::Deserialize; // Импортируем трейт и одноименный дерайв-макрос из библиотеки

/// Трейт, описывающий еду.
pub trait Food {
    /// Получить описание еды.
    fn description(&self) -> String;
}

#[derive(Deserialize)] // Реализуем трейт Deserialize с помощью макроса из библиотеки
pub struct Milk {
    liters: f32,
    fat_percentage: f32,
}

/// Обозначаем что Молоко -- это еда.
impl Food for Milk {
    /// Определение того, как выглядит описание молока.
    fn description(&self) -> String {
        format!("{} liters of {}% milk", self.liters, self.fat_percentage)
    }
}

pub struct Eggs;

impl Food for Eggs {
    fn description(&self) -> String {
        String::from("Eggs")
    }
}

pub struct Apple;

impl Food for Apple {
    fn description(&self) -> String {
        String::from("Apple")
    }
}
