pub struct Episode {
    original_file_name: Path,
    extension: None<string>,
    show_name: None<string>,
    episode_name: None<string>,
    season_number: None<int>,
    episode_number: None<int>
}

impl Episode {
    pub fn new() -> Episode {
        Episode { original_file_name, extension: None, show_name: None, episode_name: None,
        season_number: None, episode_number: None }
    }

    pub fn rename(&self) {

    }
}