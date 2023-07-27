use fridge_emulator::*;

fn main() {
    let mut fridge = fridge::Fridge::new(); // Создаем холодильник

    let stdin = std::io::stdin(); // Получаем специальный объект для считывания ввода пользователя

    for _ in 0..3 { // Даем три попытки на ввод еды
        println!("Enter food name:");

        let mut name_line = String::new();
        stdin.read_line(&mut name_line).unwrap(); // Считываем строку от пользователя
        let name = name_line.trim(); // Убираем лишний перенос строки в конце и пр.

        match name {
            "milk" => fridge.put(read_object::<food::Milk>(&stdin)),
            "eggs" => fridge.put(food::Eggs),
            "apple" => fridge.put(food::Apple),
            unknown => println!("I don't know what to do with `{unknown}`"),
        }
        println!();
    }

    println!("Fridge: {}", fridge);
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
