use std::convert::Infallible;

pub trait Template {
    /// This function will simply return a boolean of if the template should handle the type of
    /// page that is currently being requested.
    #[inline]
    #[allow(unused_variables)]
    fn should_handle_page(page_type: String) -> bool {
        true
    }

    /// This function will request the data to be used by the template.
    fn construct_theme_data(database: Infallible) -> dyn ThemeData;
}

/// This is the object that contain keys and values that is accessible inside the template.
pub trait ThemeData {
    fn get(key: impl Into<String>) -> String
    where
        Self: Sized;
}
