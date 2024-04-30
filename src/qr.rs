use qrcode::{types::{Color, QrError}, QrCode};

use crate::matrix::Matrix;

#[allow(missing_debug_implementations)]
pub struct Qr {
    code: QrCode,
}

impl Qr {
    pub fn from<T: AsRef<[u8]>>(data: T) -> Result<Self, QrError> {
        Ok(Self {
            code: QrCode::new(data.as_ref())?,
        })
    }

    pub fn to_matrix(&self) -> Matrix<Color> {
        Matrix::new(self.code.to_colors())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn print_qr_too_long() {
        Qr::from(&String::from_utf8(vec![b'a'; 8000]).unwrap()).unwrap();
    }
}