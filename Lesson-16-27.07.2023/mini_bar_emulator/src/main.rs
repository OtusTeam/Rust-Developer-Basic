use mini_bar_emulator::*;

fn main() {
    let mut mini_bar = fridge::DrinkBar::new();
    let mut fruit_bar = fridge::FruitBar::new();

    let stdin = std::io::stdin(); // Получаем специальный объект для считывания ввода пользователя

    println!("Filling Mini Bar:");
    fill_bar(&mut mini_bar, 3, &stdin);

    println!("\nFilling Fruit Bar:");
    fill_bar(&mut fruit_bar, 3, &stdin);

    println!("\nMini Bar: {}", mini_bar);
    println!("\nFruit Bar: {}", fruit_bar);
}

/// Заполняем холодильник `bar` едой `n` раз
fn fill_bar<B, F>(bar: &mut B, n: usize, stdin: &std::io::Stdin)
where
    B: fridge::Bar<Item = F>, // Требуем, что `B` -- это холодильник, который хранит `F`
    F: std::fmt::Display + serde::de::DeserializeOwned, // Требуем, что `F` умеет красиво печататься и десериализовываться
{
    for _ in 0..n {
        let food = read_object::<F>(stdin); // Читаем еду
        println!("{} has been put into bar", food); // Логируем
        bar.put(food); // Добавляем еду в холодильник
    }
}

/// Прочитать объект из стандартного ввода в формате JSON на одной строке
fn read_object<T: serde::de::DeserializeOwned>(stdin: &std::io::Stdin) -> T {
    println!("Enter object in JSON:");

    // Получаем строку из стандартного ввода
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();

    // Десериализуем объект из строки в формате JSON
    serde_json::from_str(&line).unwrap()
}
