use std::error::Error;
use std::sync::Arc;
use headless_chrome::{Browser, Tab};
use headless_chrome::protocol::cdp::Page;
use std::{thread, time::Duration};
use chrono::prelude::*;

use crate::display::clear; // Import the Display struct
use crate::web_scraper::WebScraper; // Import the web_scraper class 

pub fn get_data() -> String {    

    // Creating an instance of WebScraper for panel 1
    let panel1 = WebScraper::new(
        1,
        "https://getbootstrap.com/docs/5.0/content/images/".to_string(),
        "input#search-input".to_string(),
        "Responsive".to_string(),
        "".to_string(),
        0,
        "screenshot.jpeg".to_string(),
        "screenshot.png".to_string(),
    );

    // Creating an instance of WebScraper for panel 2
    let panel2 = WebScraper::new(
        2,
        "https://www.rust-lang.org/".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        0,
        "creenshot2.jpeg".to_string(),
        "screenshot2.png".to_string(),
    );

    // Creating an instance of WebScraper for panel 3
    let panel3 = WebScraper::new(
        3,
        "https://www.bing.com/".to_string(),
        "#sb_form_q".to_string(),
        "premier league football scores table".to_string(),
        "#b_results > li > div".to_string(),
        4,
        "screenshot3.jpeg".to_string(),
        "screenshot3.png".to_string(),
    );    

    // Screenshot a pages.
    assert!(browser_search(panel1).is_ok());

    assert!(browser_search(panel2).is_ok());

    assert!(browser_search(panel3).is_ok());

    let local_time: DateTime<Local> = Local::now();
    return format!("{}", local_time.format("%Y-%m-%d %H:%M:%S"))
}

fn browser_search(webpage : WebScraper) -> Result<(), Box<dyn Error>> {
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;
    let path_name = r#"..\src\assets\"#;

    match clear(path_name, &webpage.image_home, &webpage.image_element) {
        Ok(()) => println!("Images processed successfully!"),
        Err(e) => eprintln!("Error: {}", e),
    }
    
    // Navigate to URL
    let found_url = match tab.navigate_to(&webpage.url) {
        Ok(_) => {
            println!("Successfully navigated to URL: {}", &webpage.url);
            true
        }
        Err(e) => {
            println!("Failed to navigate to URL: {} - {}", &webpage.url, e);
            false
        }
    };
        
    if found_url
    {
        if !webpage.search_box.is_empty() {
            println!("Checking for search box: {}", webpage.search_box);
            let test = find_element_check(tab.clone(), &webpage.search_box);
            println!("Search box check result: {}", test);
            if test.contains("Found Element") {
                tab.wait_for_element(&webpage.search_box)?.click()?;  // Wait for network/javascript/dom to make the search-box available and click it.
                tab.type_str(&webpage.search_text)?.press_key("Enter")?; // Type in a query and press `Enter`
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
            println!("Saving screenshot as {}", webpage.image_home);
            std::fs::write(format!("{}{}", path_name, webpage.image_home), jpeg_data)?;
        }    

        if webpage.delay_search > 0 { //delay after search 
            println!("Delaying for {} milliseconds...", webpage.delay_search);
            thread::sleep(Duration::from_millis(4)); 
        }

        if !webpage.search_element.is_empty() { //test may have to add this https://developer.mozilla.org/en-US/docs/Web/HTML (it only picks up these tags)
            println!("Checking for element: {}", webpage.search_element);
            let test = find_element_check(tab.clone(), &webpage.search_element);
            println!("Element check result: {}", test);
            if test.contains("Found Element") {
                println!("Capturing screenshot of specific element...");
                // Take a screenshot of just the one part 
                let png_data = tab.wait_for_element(&webpage.search_element)?
                    .capture_screenshot(Page::CaptureScreenshotFormatOption::Png)?;
                println!("Saving element screenshot as {}", webpage.image_element);
                // Save the screenshot to disc
                std::fs::write(format!("{}{}", path_name, webpage.image_element), png_data)?;
            }
        }
    }
    
    webpage.display_parameters();
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