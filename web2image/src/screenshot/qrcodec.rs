use anyhow::Result;
use image::Luma;
use qrcode::QrCode;

const QRCODE_FILE_PATH: &str = "qrcode.png";
pub fn generate_qr_code(url: &str, output: &str) -> Result<()> {
    let code = QrCode::new(url)?;
    let image = code.render::<Luma<u8>>().build();
    let path = format!("{}/{}", output, QRCODE_FILE_PATH);
    image.save(path)?;
    Ok(())
}
