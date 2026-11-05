use std::path::PathBuf;

#[derive(Debug)]
pub struct GlobalContext {
    home_path: PathBuf,
    base_path: PathBuf,
    tracker_path: PathBuf,
}

impl GlobalContext {
    pub fn new(home_dir: PathBuf) -> Self {
        let base_path = home_dir.join(".fintrack");
        let tracker_path = base_path.join("tracker.json");

        GlobalContext {
            home_path: home_dir,
            base_path,
            tracker_path,
        }
    }

    pub fn tracker_path(&self) -> &PathBuf {
        &self.tracker_path
    }

    pub fn home_path(&self) -> &PathBuf {
        &self.home_path
    }

    pub fn base_path(&self) -> &PathBuf {
        &self.base_path
    }
}
