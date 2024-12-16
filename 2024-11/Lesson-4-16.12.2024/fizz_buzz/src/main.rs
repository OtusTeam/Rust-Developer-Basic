// Разминочное задание, классическая задачка с собеседований,
// которая обычно дается, чтобы проверить может ли кандидат писать самый базовый код
// На вход подается число n
// Программа перебирает все числа от 1 до n (включая n) и печатает:
// Если число делится только на 3 то печатается Fizz
// Если число делится только на 5 то печатается Buzz
// Если число делится на 3 и на 5 то печатается FizzBuzz
// Если число не делится ни на 3, ни на 5, то печатается само число

// Пример:
// На вход:
// 5
// На выходе:
// 1
// 2
// Fizz
// Buzz
// Fizz

fn fizzbuzz<W: std::io::Write>(n: i32, writer: &mut W) {
    // ваш код
    // печатать с помощью writeln!(w, ...).unwrap();
    // обычно в стандартный вывод мы печатаем с помощью макроса println!()
    // но тут мы печатаем в произвольный Write (в основной программе это будет стандартный вывод, а в тестах тестовый буффер)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fizz_buzz() {
        let mut buf = std::io::BufWriter::new(Vec::new());
        fizzbuzz(15, &mut buf);
        assert_eq!(
            String::from_utf8(buf.into_inner().unwrap()).unwrap(),
            "1\n2\nFizz\n4\nBuzz\nFizz\n7\n8\nFizz\nBuzz\n11\nFizz\n13\n14\nFizzBuzz\n"
        );
    }
}

fn main() {
    let mut input_line = String::new();
    std::io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    let n: i32 = input_line.trim().parse().expect("Input not an integer");
    fizzbuzz(n, &mut std::io::stdout());
}
