use std::{error::Error, fs::File, io::Write, time::Duration};
use headless_chrome::{browser, protocol::cdp::Page};

pub fn check_login(tab: &browser::Tab) -> Result<bool, Box<dyn Error>> {
    let mut test_bool = false;

    // Define the two selectors
    let selector_one = "._akaz";
    let selector_two = ".x1y332i5";

    // Continuously check for the presence of either element
    loop {
        // Try to find the first element
        if let Ok(_) = tab.wait_for_element_with_custom_timeout(selector_one, std::time::Duration::from_secs(1)) {
            test_bool = false; // Found the first element
            break;
        }

        // Try to find the second element
        if let Ok(_) = tab.wait_for_element_with_custom_timeout(selector_two, std::time::Duration::from_secs(1)) {
            test_bool = true; // Found the second element
            break;
        }
    }

    Ok(test_bool)
}

pub fn login(tab: &browser::Tab) -> Result<(), Box<dyn Error>> {
    tab.wait_for_element("._akaz")?;

    let viewport = tab.wait_for_element("._akau")?
        .get_box_model()?
        .content_viewport();
    
    let _png_data = tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Jpeg, None, None, true)?;

    let mut file = File::create("QR.jpg")?;
    file.write_all(&_png_data)?;

    println!("Scan From the QR code from QR.JPEG");

    let chatEle = tab.wait_for_element_with_custom_timeout(".x1y332i5", Duration::from_millis(160000));
    match chatEle {
        Ok(element) => {
            println!("Successfully Accessed Your WhatsApp");
        },
        Err(e) => {
            println!("{}", e);
        }
    }

    Ok(())
}