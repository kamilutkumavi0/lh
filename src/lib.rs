pub mod file_reader;
pub mod filter_output;
pub mod output_printer;
pub mod parserer;
pub mod tomlread;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parserer_default_values() {
        // Test that parser has reasonable defaults
        use parserer::{PType, SortType};
        
        // Test enum values exist
        let _ = PType::All;
        let _ = PType::File;
        let _ = PType::Dir;
        
        let _ = SortType::Name;
        let _ = SortType::Size;
    }

    #[test]
    fn test_file_reader_element_creation() {
        use file_reader::Element;
        
        // Test Element::new function
        let element = Element::new(
            "test".to_string(),
            true,  // is_file
            false, // is_dir
            false, // is_sym
            None,  // file_type
        );
        
        assert_eq!(element.name, "test");
        assert_eq!(element.is_file, true);
        assert_eq!(element.is_dir, false);
        assert_eq!(element.is_sym, false);
        assert_eq!(element.is_hidden, false);
    }

    #[test]
    fn test_toml_reading_non_existing_config() {
        use tomlread::toml_read;
        
        // This should not panic and return default configuration
        let config = toml_read();
        
        // Should contain at least default config entries
        assert!(config.contains_key("default"));
    }
}
