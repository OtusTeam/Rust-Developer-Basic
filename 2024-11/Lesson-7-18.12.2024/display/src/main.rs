// Вам нужно реализовать программу обработки команд для дисплея.
// На вход пользователь подает:
// * 2 числа: размер дисплея
// * 1 число: цвет дисплея по-умолчанию (1 - красный, 2 - зеленый, 3 - синий)
// * Последовательность команд: набор чисел.
//
// Дисплей поддерживает следующие команды:
// * 1 x y - переместить курсор в позицию x y
// * 2 colour - перекрасить пиксель в цвет colour
//
// Пример входных данных:
// 4 4
// 1
// 1 2 2 2 3
// В результате пиксель по позиции (2,2) будет перекрашен в синий цвет

// Обновлять состояние дисплея нужно через метод matrix.set_colour(pos_x, pos_y, colour)

// Важно! Обязательна проверка на ошибки. Если пользователь просит переместиться на пиксель за пределами дисплея или ввел неправильный цвет, то вам нужно кинуть панику!

use std::io::{self, Write};
mod matrix;
use matrix::Matrix;

struct Display {
    // можете добавить сюда любые дополнительные поля
    matrix: Matrix,
    cursor: Cursor,
    width: u64,
    height: u64,
}

impl Display {
    fn move_cursor(&mut self, x: u64, y: u64) {
        if x >= self.width || y >= self.height {
            panic!("Координаты ({}, {}) выходят за пределы дисплея!", x, y);
        }
        self.cursor.x = x;
        self.cursor.y = y;
    }

    fn set_colour(&mut self, colour: u8) {
        if !(1..=3).contains(&colour) {
            panic!("Некорректный цвет: {}", colour);
        }
        self.matrix.set_colour(self.cursor.x, self.cursor.y, colour);
    }
}

struct Cursor {
    x: u64,
    y: u64,
}

fn create_display(max_width: u32, max_height: u32, default_colour: u8) -> Display {
    // ваш код сюда
    Display {
        matrix: Matrix::new(max_width, max_height, default_colour),
        cursor: Cursor { x: 0, y: 0 },
        width: max_width as u64,
        height: max_height as u64,
    }
}

fn command_parser(unfiltered_commands_input: Vec<u64>) -> Vec<u64> {
    let mut parsed_commands = Vec::new();
    let mut i = 0;

    while i < unfiltered_commands_input.len() {
        match unfiltered_commands_input[i] {
            1 => {
                if i + 2 < unfiltered_commands_input.len() {
                    let x = unfiltered_commands_input[i + 1];
                    let y = unfiltered_commands_input[i + 2];

                    // Добавляем команду MoveCursor
                    parsed_commands.push(1);
                    parsed_commands.push(x);
                    parsed_commands.push(y);

                    i += 3;
                } else {
                    break;
                }
            }
            2 => {
                if i + 1 < unfiltered_commands_input.len() {
                    let color = unfiltered_commands_input[i + 1] as u8;

                    // Добавляем команду SetColour
                    parsed_commands.push(2);
                    parsed_commands.push(color as u64);

                    i += 2;
                } else {
                    break;
                }
            }
            _ => {
                i += 1;
            }
        }
    }

    parsed_commands
}

fn process_commands(display: &mut Display, input: Vec<u64>) {
    let mut i = 0;

    while i < input.len() {
        match input[i] {
            1 => {
                if i + 2 < input.len() {
                    let x = input[i + 1];
                    let y = input[i + 2];
                    display.move_cursor(x, y);
                    i += 3;
                } else {
                    panic!("Некорректная команда для перемещения курсора");
                }
            }
            2 => {
                if i + 1 < input.len() {
                    let colour = input[i + 1] as u8;
                    display.set_colour(colour);
                    i += 2;
                } else {
                    panic!("Некорректная команда для установки цвета");
                }
            }
            _ => {
                panic!("Неизвестная команда: {}", input[i]);
            }
        }
    }
}

// код ниже трогать не нужно, можете просто посмотреть его

// тесты
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_happy_case() {
        let mut display = create_display(4, 4, 1);
        process_commands(&mut display, vec![1, 2, 2, 2, 3]);
        let mut expected = Matrix::new(4, 4, 1);
        expected.set_colour(2, 2, 3);
        assert_eq!(display.matrix, expected);
    }

    #[test]
    #[should_panic]
    fn test_error() {
        let mut display = create_display(4, 4, 1);
        process_commands(&mut display, vec![1, 5, 5, 2, 3]);
    }

    #[test]
    #[should_panic]
    fn test_error_invalid_colour() {
        let mut display = create_display(4, 4, 1);
        process_commands(&mut display, vec![1, 2, 2, 2, 5]);
    }
}

fn main() {
    print!("Введите размеры дисплея (ширина высота): ");
    io::stdout().flush().unwrap(); // Сбрасываем буфер вывода, чтобы сообщение отобразилось сразу
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let (width, height) = parse_dimensions(&input);

    print!("Введите стандартный цвет дисплея (1 - красный, 2 - зеленый, 3 - синий): ");
    io::stdout().flush().unwrap(); // Сбрасываем буфер вывода, чтобы сообщение отобразилось сразу
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let default_colour = match input.trim() {
        "1" => 1, // Красный
        "2" => 2, // Зеленый
        "3" => 3, // Синий
        _ => panic!("Неверный ввод цвета. Ожидалось 1, 2 или 3."),
    };

    // Создаём дисплей и заполняем его стандартным цветом
    let mut display = create_display(width, height, default_colour);

    // Ввод действий
    print!("Введите строку с действиями: ");
    io::stdout().flush().unwrap(); // Сбрасываем буфер вывода, чтобы сообщение отобразилось сразу
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let unspecified_user_input = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Обработка введенных команд
    let commands = command_parser(unspecified_user_input);

    // Отображение дисплея
    process_commands(&mut display, commands);

    display.matrix.display();
}

fn parse_dimensions(input: &str) -> (u32, u32) {
    let parts: Vec<u32> = input
        .split_whitespace()
        .map(|x| x.parse().expect("Неверный ввод размера"))
        .collect();
    if parts.len() != 2 {
        panic!("Ожидалось два числа для размеров дисплея.");
    }
    (parts[0], parts[1])
}
