#[derive(PartialEq, Debug)]
pub struct Matrix(Vec<Vec<u8>>);

fn color_to_char(color: u8) -> char {
    match color {
        1 => '\u{1F534}', // Красный кружок
        2 => '\u{1F7E2}', // Зелёный кружок
        3 => '\u{1F535}', // Синий кружок
        _ => ' ',
    }
}

impl Matrix {
    pub fn new(width: u32, height: u32, default_color: u8) -> Self {
        Self(vec![vec![default_color; width as usize]; height as usize])
    }

    pub fn display(&self) {
        for row in &self.0 {
            for &cell in row {
                print!("{}", color_to_char(cell));
            }
            println!();
        }
    }

    pub fn set_colour(&mut self, x: u64, y: u64, colour: u8) {
        self.0[x as usize][y as usize] = colour;
    }
}
