// Обобщаем поведение с помощью шаблонов и статического полиморфизма.
// Пусть у нас 3 типа фигур: треугольник, прямоугольник и круг
// Создайте трейт Shape, в котором есть методы:
// get_area(&self) -> f64 // возвращает зачение площади фигуры
// get_perimeter(&self) -> f64 // возвращает значение периметра фигуры
// Реализуйте данный трейт для треугольника, прямоугольника и круга

// Напишите 1 функцию perimeter_by_area, которая может принимать любую фигуру
// и возвращает отнощение ее периметра к площади (P/A)

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
            perimeter_by_area(Triangle {
                sides_lens: [3.0, 4.0, 5.0]
            }),
            2.0
        );
        relative_eq!(perimeter_by_area(Circle { radius: 2.0 }), 1.0);
        relative_eq!(
            perimeter_by_area(Rectangle {
                width: 2.0,
                height: 3.0,
            }),
            1.6666
        );
    }
}
