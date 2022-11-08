mod qrcodec;

pub use qrcodec::generate_qr_code;

use anyhow::{anyhow, Result};
use headless_chrome::{protocol::page::ScreenshotFormat, Browser, LaunchOptionsBuilder};
use std::fs;

const FILE_NAME: &str = "screenshot.png";
pub fn save_screen_shot(url: &str, output: &str) -> Result<()> {
    let options = LaunchOptionsBuilder::default()
        // Make the window bigger
        .window_size(Some((1920, 1600)))
        .build()
        .expect("Couldn't find appropriate Chrome binary.");

    let browser = Browser::new(options).map_err(|e| anyhow!(e.to_string()))?;

    // init tab
    let tab = browser
        .wait_for_initial_tab()
        .map_err(|e| anyhow!(e.to_string()))?;
    // navigate to url
    tab.navigate_to(url).map_err(|e| anyhow!(e.to_string()))?;
    tab.wait_until_navigated()
        .map_err(|e| anyhow!(e.to_string()))?;

    // screenshot
    let png_data = tab
        .capture_screenshot(ScreenshotFormat::PNG, None, true)
        .map_err(|e| anyhow!(e.to_string()))?;

    let path = format!("{}/{}", output, FILE_NAME);
    fs::write(path, png_data)?;
    Ok(())
}
