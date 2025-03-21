use serde::{Serialize, Deserialize};

use std::error::Error;
use std::sync::Arc;
use headless_chrome::{Browser, Tab};
use headless_chrome::protocol::cdp::Page;
use std::{thread, time::Duration};
use chrono::prelude::*;

use crate::display::clear; // Import the Display struct

#[derive(Debug, Serialize, Deserialize)]
pub struct Panel {
    pub id: i32,
    pub url: String,
    pub search_box : String, //format parent#node_id e.g., input#id
    pub search_text : String,
    pub search_element : String, //format parent#node_id div#id and has to be HTML elements (https://developer.mozilla.org/en-US/docs/Web/HTML) can have other formats #mw-content-text > div > table.infobox.vevent
    pub image_home : String,
    pub image_element : String,
    pub image_home_comment : String, //e.g., if item was found or not 
    pub image_element_comment : String, //e.g., if item was found or not 
}

pub fn get_data() -> String {    
    // Screenshot a pages.
    assert!(browser_search("https://getbootstrap.com/docs/5.0/content/images/", "input#search-input", "Responsive", "", 0 , "screenshot.jpeg", "screenshot.png").is_ok());

    assert!(browser_search("https://www.rust-lang.org/", "", "", "", 0, "screenshot2.jpeg", "screenshot2.png").is_ok());

    assert!(browser_search("https://www.bing.com/", "#sb_form_q", "premier league football scores table", "#b_results > li > div", 4, "screenshot3.jpeg", "screenshot3.png").is_ok());

    let local_time: DateTime<Local> = Local::now();
    return format!("{}", local_time.format("%Y-%m-%d %H:%M:%S"))
}

fn browser_search(find_url : &str, find_box : &str, find_text : &str, find_element : &str, delay_search : i32, image_home_name : &str, image_element_name : &str) -> Result<(), Box<dyn Error>> {
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;
    let path_name = r#"..\src\assets\"#;

    match clear(path_name, image_home_name, image_element_name) {
        Ok(()) => println!("Images processed successfully!"),
        Err(e) => eprintln!("Error: {}", e),
    }
    
    // Navigate to URL
    let found_url = match tab.navigate_to(find_url) {
        Ok(_) => {
            println!("Successfully navigated to URL: {}", find_url);
            true
        }
        Err(e) => {
            println!("Failed to navigate to URL: {} - {}", find_url, e);
            false
        }
    };
        
    if found_url
    {
        if !find_box.is_empty() {
            println!("Checking for search box: {}", find_box);
            let test = find_element_check(tab.clone(), find_box);
            println!("Search box check result: {}", test);
            if test.contains("Found Element") {
                tab.wait_for_element(find_box)?.click()?;  // Wait for network/javascript/dom to make the search-box available and click it.
                tab.type_str(find_text)?.press_key("Enter")?; // Type in a query and press `Enter`
            }
        }

        // Take a screenshot of the entire browser window
        let jpeg_data =     
        match tab.capture_screenshot(
            Page::CaptureScreenshotFormatOption::Jpeg,
            None, None, true,)  {
            Ok(_k) => _k,
            Err(_e) => Vec::new(), //empty
        };

        // Check if jpeg_data contains data (is not empty)
        if jpeg_data.is_empty() {
            println!("Screenshot data is empty. Something went wrong!");
        } else {
            println!("Saving screenshot as {}", image_home_name);
            std::fs::write(format!("{}{}", path_name, image_home_name), jpeg_data)?;
        }    

        if delay_search > 0 { //delay after search 
            println!("Delaying for {} milliseconds...", delay_search);
            thread::sleep(Duration::from_millis(4)); 
        }

        if !find_element.is_empty() { //test may have to add this https://developer.mozilla.org/en-US/docs/Web/HTML (it only picks up these tags)
            println!("Checking for element: {}", find_element);
            let test = find_element_check(tab.clone(), find_element);
            println!("Element check result: {}", test);
            if test.contains("Found Element") {
                println!("Capturing screenshot of specific element...");
                // Take a screenshot of just the one part 
                let png_data = tab.wait_for_element(find_element)?
                    .capture_screenshot(Page::CaptureScreenshotFormatOption::Png)?;
                println!("Saving element screenshot as {}", image_element_name);
                // Save the screenshot to disc
                std::fs::write(format!("{}{}", path_name, image_element_name), png_data)?;
            }
        }
    }
    
    println!("End for browser_search function.");
    Ok(())
}

fn find_element_check(tab: Arc<Tab>, element: &str) -> String{
    let mut _out_put = "";
    // Specify a course of action for each case.
    match tab.wait_for_element(element) {
        Ok(_k) => _out_put = "Found Element",
        Err(_e) => _out_put = "Could not find element",
    }

    println!("{} : {}", element, _out_put);
    return format!("{}", _out_put);
}