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

use std::io;
mod matrix;
use matrix::Matrix;

#[derive(Default)]
struct Point {
    pos_x: u32,
    pos_y: u32,
}

struct Display {
    matrix: Matrix,
    cursor: Point,
    width: u32,
    height: u32,
}

fn create_display(max_width: u32, max_height: u32, default_colour: u8) -> Display {
    Display {
        matrix: Matrix::new(max_width, max_height, default_colour),
        cursor: Point::default(),
        width: max_width,
        height: max_height,
    }
}

fn process_commands(display: &mut Display, input: Vec<u64>) {
    let mut it_input = input.into_iter();

    while let Some(action) = it_input.next() {
        match action {
            1 => change_cursor_position(
                display,
                it_input
                    .next()
                    .zip(it_input.next())
                    .expect("Неверные формат координат курсора"),
            ),
            2 => change_cursor_color(display, it_input.next().expect("Не указан цвет")),
            _ => panic!("Неверная команда"),
        }
    }
}

fn change_cursor_position(display: &mut Display, (pos_x, pos_y): (u64, u64)) {
    if pos_x < display.width as u64 && pos_y < display.height as u64 {
        display.cursor = Point {
            pos_x: pos_x as u32,
            pos_y: pos_y as u32,
        };
    } else {
        panic!("Курсор не должен выходить за границы экрана")
    }
}

fn change_cursor_color(display: &mut Display, color: u64) {
    match color {
        1..=3 => display.matrix.set_colour(
            display.cursor.pos_x as u64,
            display.cursor.pos_y as u64,
            color as u8,
        ),
        _ => panic!("Неверный цвет"),
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
    println!("Введите размеры дисплея (ширина высота):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let (width, height) = parse_dimensions(&input);

    println!("Введите стандартный цвет дисплея (1 - красный, 2 - зеленый, 3 - синий):");
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
    println!("Введите строку с действиями:");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let commands = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    // Отображение дисплея
    process_commands(&mut display, commands);

    display.matrix.display();
}

fn parse_dimensions(input: &str) -> (u32, u32) {
    let parts: Vec<u32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Неверный ввод размера"))
        .collect();
    if parts.len() != 2 {
        panic!("Ожидалось два числа для размеров дисплея.");
    }
    (parts[0], parts[1])
}
