use qrcode::types::QrError;
use qrsmth_lib::{generate_qr_string, print_qr};

fn main() -> Result<(), QrError> {
    // print_qr("www.google.com")?;
    generate_qr_string("www.google.com")?;
    Ok(())
}