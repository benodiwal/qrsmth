use qrcode::types::QrError;

pub mod util;
pub mod matrix;
pub mod qr;
pub mod render;

const QUIET_ZONE_WIDTH: usize = 2;

pub fn print_qr<D: AsRef<[u8]>>(data: D) -> Result<(), QrError> {
    let mut matrix = qr::Qr::from(data)?.to_matrix();
    matrix.surround(QUIET_ZONE_WIDTH, render::QrLight);

    render::Renderer::default().print_stdout(&matrix);
    Ok(())
}

pub fn generate_qr_string<D: AsRef<[u8]>>(data: D) -> Result<String, QrError> {
    let mut matrix = qr::Qr::from(data)?.to_matrix();
    matrix.surround(QUIET_ZONE_WIDTH, render::QrLight);

    let mut buf = Vec::new();
    render::Renderer::default().render(&matrix, &mut buf).expect("failed to generate qr string");

    Ok(String::from_utf8(buf).unwrap())
}