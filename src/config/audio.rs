use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Audio {
    pub enabled: bool,
}

impl Default for Audio {
    fn default() -> Self {
        Audio { enabled: true }
    }
}
