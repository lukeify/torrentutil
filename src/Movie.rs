pub struct Movie {
    original_file_name: Path,
    extension: String,
    name: Option<String>,
    year: Option<int>
}

impl Movie {
    pub fn new(original_file_name: Path) -> Movie {
        Movie { original_file_name, extension: None, name: None, year: None }
    }

    pub fn rename(&self) {

    }
}