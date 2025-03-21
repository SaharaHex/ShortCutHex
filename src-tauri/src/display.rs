use image::open;
use std::path::Path;

pub fn clear(path_name: &str, image_home_name: &str, image_element_name: &str,) -> Result<(), Box<dyn std::error::Error>> 
{
    let image_clear_home = "notfound.jpeg";
    let image_clear_element = "notfound.png";

    // Ensure the directory exists
    if !Path::new(path_name).exists() {
        return Err(format!("Path '{}' does not exist!", path_name).into());
    }

    // Load the "home" image
    let new_img_home = match open(format!("{}{}", path_name, image_clear_home)) {
        Ok(img) => img,
        Err(e) => {
            return Err(format!("Failed to open the new image - jpeg: {}",e).into())
        }
    };

    // Load the "element" image
    let new_img_element = match open(format!("{}{}", path_name, image_clear_element)) {
        Ok(img) => img,
        Err(e) => {
            return Err(format!("Failed to open the new image - png: {}",e).into())
        }
    };

    // Replace the "home" image
    if let Err(e) = new_img_home.save(format!("{}{}", path_name, image_home_name)) {
        return Err(format!("Failed to replace the image - jpeg: {}",e).into());
    }

    // Replace the "element" image
    if let Err(e) = new_img_element.save(format!("{}{}", path_name, image_element_name)) {
        return Err(format!("Failed to replace the image - png: {}",e).into());
    }

    Ok(())
}
