use cli::Args;
use qrcode::types::QrError;
use qrsmth_lib::print_qr;

mod cli;

fn main() -> Result<(), QrError> {
    let args = Args::parse_args();
    let input = args.input.unwrap();
    print_qr(input)?;

    Ok(())
}