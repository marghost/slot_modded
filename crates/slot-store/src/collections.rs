use std::io;
use std::path::Path;

use crate::{Cart, Platform, Recent, SlotState, CONFIG_DIR};

pub const LEGACY_FAVORITES_FILE: &str = "favourites.txt";

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct CartKey {
    pub platform: Platform,
    pub stem: String,
}

impl CartKey {
    pub fn of(cart: &Cart) -> Self {
        CartKey {
            platform: cart.platform,
            stem: cart.stem.clone(),
        }
    }

    pub fn is(&self, cart: &Cart) -> bool {
        self.platform == cart.platform && self.stem == cart.stem
    }

    fn parse(line: &str) -> Option<Self> {
        let (dir, stem) = line.trim_end_matches('\r').split_once('/')?;
        let platform = Platform::ALL
            .into_iter()
            .find(|p| p.dir_name().eq_ignore_ascii_case(dir))?;
        if stem.trim().is_empty() || stem.contains(['/', '\\', '\0']) {
            return None;
        }
        Some(CartKey {
            platform,
            stem: stem.to_string(),
        })
    }
}

pub fn import_legacy_favorites(root: &Path, state: &mut SlotState) -> io::Result<bool> {
    if !root.join(CONFIG_DIR).join(LEGACY_FAVORITES_FILE).exists() {
        return Ok(false);
    }
    for key in read_list(root, LEGACY_FAVORITES_FILE)? {
        if !state.is_favorite(key.platform, &key.stem) {
            state.favorites.push(Recent {
                platform: key.platform,
                stem: key.stem,
            });
        }
    }
    Ok(true)
}

pub fn remove_legacy_favorites(root: &Path) -> io::Result<()> {
    std::fs::remove_file(root.join(CONFIG_DIR).join(LEGACY_FAVORITES_FILE))
}

fn read_list(root: &Path, file: &str) -> io::Result<Vec<CartKey>> {
    let text = match std::fs::read_to_string(root.join(CONFIG_DIR).join(file)) {
        Ok(text) => text,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(e),
    };
    let mut keys: Vec<CartKey> = Vec::new();
    for key in text.lines().filter_map(CartKey::parse) {
        if !keys.contains(&key) {
            keys.push(key);
        }
    }
    Ok(keys)
}
