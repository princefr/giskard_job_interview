


pub struct Utils {}

impl Utils {
    /*!
     * Get the path from a link
     * @param link: String
     * @return String
     */
    pub fn get_absolute_path(link: String) -> String {
        if let Some(index) = link.rfind("/") {
            let result = &link[..index];
            return result.to_string();
        }
        return link;
    }
}