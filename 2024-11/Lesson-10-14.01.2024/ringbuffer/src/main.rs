// Слайсы. (мы спрашиваем эту задачку на собеседования на уровено Junior Engineer)
// Ring Buffer (кольцевой буффер) - структура данных, которая позволяет очень удобно реализовывать очередь на массиве фиксированного размера.
// https://ru.wikipedia.org/wiki/%D0%9A%D0%BE%D0%BB%D1%8C%D1%86%D0%B5%D0%B2%D0%BE%D0%B9_%D0%B1%D1%83%D1%84%D0%B5%D1%80
// Ключевая идея в том, что заполняя буффер до конца мы переходим в начало
// Пример API, вызовов и как меняется состояние буффера:
// [ _ _ _ ] create(3)
// [ a b _ ] write "ab" -> return 2
// [ a b c ] write "cd" -> return 1
// [ _ b c ] read(1) -> return "a"
// [ e b c ] write "e" -> return 1
// [ e _ _ ] read(2) -> return "bc"
// Ваша задача написать такой буффер и добавить тесты

struct RingBuffer {
    read_idx: usize,
    write_idx: usize,
    data: Vec<u8>,
}

fn create(size: usize) -> RingBuffer {
}

fn write(rb: &mut RingBuffer, ...) -> usize {
}

fn read(rb: &mut RingBuffer, ...) -> ... {
}

#[cfg(test)]
mod tests {
    use super::*;

    // обязательно добавьте тесты
    #[test]
    fn test_1() {
    }
}
