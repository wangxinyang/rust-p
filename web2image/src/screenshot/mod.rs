mod qrcodec;

pub use qrcodec::generate_qr_code;

use anyhow::Result;
use headless_chrome::{protocol::page::ScreenshotFormat, Browser};
use std::fs;

const FILE_NAME: &str = "screenshot.png";
pub fn save_screen_shot(url: &str, output: &str) -> Result<()> {
    let browser = Browser::default().unwrap();

    // init tab
    let tab = browser.wait_for_initial_tab().unwrap();
    // navigate to url
    tab.navigate_to(url).unwrap();
    tab.wait_until_navigated().unwrap();

    // screenshot
    let png_data = tab
        .capture_screenshot(ScreenshotFormat::PNG, None, true)
        .unwrap();

    let path = format!("{}/{}", output, FILE_NAME);
    fs::write(path, png_data)?;
    Ok(())
}
