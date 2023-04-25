/// Trait to define terminal user interface functionnalities
/// that Tazui application needs.
pub trait Tui {
    fn init(&mut self) -> Result<(), std::io::Error>;
    fn get_start_of_line(&self) -> String;
    fn get_expression(&mut self, history: &Vec<String>) -> Result<String, std::io::Error>;
    fn display_text(&mut self, text: &String) -> Result<(), std::io::Error>;
    fn display_text_with_new_line(&mut self, text: &String) -> Result<(), std::io::Error>;
}
