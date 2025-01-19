// Возьмите код из предыдущего занятия.
// И теперь представим, что список фигур для которых мы хотим выполнить вычисления
// неизвестен на этапе компиляции программы.

// Исправьте фунцию perimeter_by_area, так чтобы она принимала параметр Box<dyn Shape>

struct Triangle {
    sides_lens: [f64; 3],
}

struct Rectangle {
    width: f64,
    height: f64,
}

struct Circle {
    radius: f64,
}

// исправьте сигнатуру и добавьте реализацию
fn perimeter_by_area(/* */) -> f64 {}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::relative_eq;

    #[test]
    fn test() {
        relative_eq!(
            perimeter_by_area(Box::new(Triangle {
                sides_lens: [3.0, 4.0, 5.0]
            })),
            2.0
        );
        relative_eq!(perimeter_by_area(Box::new(Circle { radius: 2.0 })), 1.0);
        relative_eq!(
            perimeter_by_area(Box::new(Rectangle {
                width: 2.0,
                height: 3.0,
            })),
            1.6666
        );
    }
}