#[derive(Debug, Clone, PartialEq)]
pub struct WebScraper {
    pub id: i32,
    pub url: String,
    pub search_box: String, // format: parent#node_id e.g., input#id
    pub search_text: String,
    pub search_element: String, //format parent#node_id div#id and has to be HTML elements (https://developer.mozilla.org/en-US/docs/Web/HTML) can have other formats #mw-content-text > div > table.infobox.vevent
    pub delay_search: i32,
    pub image_home: String,
    pub image_element: String,
}

impl WebScraper {
    // initialize a new instance
    pub fn new(
        id: i32,
        url: String,
        search_box: String,
        search_text: String,
        search_element: String,
        delay_search: i32,
        image_home: String,
        image_element: String,
    ) -> Self {
        Self {
            id,
            url,
            search_box,
            search_text,
            search_element,
            delay_search,
            image_home,
            image_element,
        }
    }

    // Default values for convenience //ToDo when reset 
    pub fn default() -> Self {
        Self {
            id: 0,
            url: String::from(""),
            search_box: String::from(""),
            search_text: String::from(""),
            search_element: String::from(""),
            delay_search: 0,
            image_home: String::from(""),
            image_element: String::from(""),
        }
    }

    // Method to display the details of the WebScraper instance
    pub fn display_parameters(&self) {
        println!(
            "WebScraper {{\n\
            ID: {},\n\
            URL: {},\n\
            Search Box: {},\n\
            Search Text: {},\n\
            Search Element: {},\n\
            Delay Search: {},\n\
            Image Home: {},\n\
            Image Element: {}\n\
            }}",
            self.id,
            self.url,
            self.search_box,
            self.search_text,
            self.search_element,
            self.delay_search,
            self.image_home,
            self.image_element
        );
    }

    // Method to update the search box with error handling //ToDo when update 
    pub fn update_search_box(&mut self, new_search_box: &str) -> Result<(), String> {
        if new_search_box.is_empty() {
            Err(String::from("Search box identifier cannot be empty."))
        } else {
            self.search_box = new_search_box.to_string();
            Ok(())
        }
    }
}