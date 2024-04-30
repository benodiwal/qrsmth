use crate::util::usize_sqrt;

#[derive(Debug)]
pub struct Matrix<T> {
    pixels: Vec<T>
}

impl<T> Matrix<T> {

    pub fn new(pixels: Vec<T>) -> Self {
        usize_sqrt(pixels.len());
        Self { pixels }
    }

    pub fn size(&self) -> usize {
        usize_sqrt(self.pixels.len())
    }

    pub fn pixels(&self) -> &[T] {
        &self.pixels
    }

    pub fn surround(&mut self, thickness: usize, quiet: T)
    where
      T: Copy,
    {
        let width = self.size();
        let out_width = width + thickness*2;

        let mut out = vec![quiet; out_width.pow(2)];
        for row in 0..width {
            for col in 0..width {
                let vec_pos = width*row + col;
                let out_row = row + thickness;
                let out_col = col + thickness;
                let out_pos = out_row*out_width + out_col;
                out[out_pos] = self.pixels[vec_pos];
            }
        }  

        self.pixels = out;     
    }

}

#[cfg(test)]
mod tests {
    use qrcode::types::Color::{Dark as QrDark, Light as QrLight};

    use super::*;

    #[test]
    #[should_panic]
    fn matrix_incorrect_size() {
        Matrix::new(vec![QrDark, QrDark, QrLight, QrLight, QrLight, QrDark]);
    }

    #[test]
    fn surround_quiet_normal() {
        let input = vec![
            0, 1, 2,
            3, 4, 5,
            6, 7, 8,
        ];

        let expected = vec![
            9, 9, 9, 9, 9, 9, 9, 9, 9,
            9, 9, 9, 9, 9, 9, 9, 9, 9,
            9, 9, 9, 9, 9, 9, 9, 9, 9,
            9, 9, 9, 0, 1, 2, 9, 9, 9,
            9, 9, 9, 3, 4, 5, 9, 9, 9,
            9, 9, 9, 6, 7, 8, 9, 9, 9,
            9, 9, 9, 9, 9, 9, 9, 9, 9,
            9, 9, 9, 9, 9, 9, 9, 9, 9,
            9, 9, 9, 9, 9, 9, 9, 9, 9,
        ];

        let mut matrix = Matrix::new(input);
        matrix.surround(3, 9);
        let actual = matrix.pixels();
        assert_eq!(expected, actual);
    }

    #[test]
    fn surround_quiet_empty() {
        let mut matrix = Matrix::new(vec![]);
        matrix.surround(3, 7);
        let actual = matrix.pixels();
        let expected = vec![7; (3 * 2) * (3 * 2)];
        assert_eq!(expected, actual)
    }

}