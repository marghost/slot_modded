use std::io;
use std::path::Path;

use crate::{atomic_write, sort_key, Cart, Platform, Recent, SlotState, CONFIG_DIR, RECENTS_MAX};

pub const FAVORITES_FILE: &str = "favourites.txt";
pub const RECENTS_FILE: &str = "recents.txt";

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

    fn line(&self) -> String {
        format!(
            "{}/{}\n",
            self.platform.dir_name().to_ascii_lowercase(),
            self.stem
        )
    }
}

#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct Collections {
    pub favorites: Vec<CartKey>,
}

impl Collections {
    pub fn read(root: &Path) -> io::Result<Self> {
        Ok(Collections {
            favorites: read_list(root, FAVORITES_FILE)?,
        })
    }

    pub fn is_favorite(&self, cart: &Cart) -> bool {
        self.favorites.iter().any(|k| k.is(cart))
    }

    pub fn toggle_favorite(&mut self, cart: &Cart) -> bool {
        match self.favorites.iter().position(|k| k.is(cart)) {
            Some(i) => {
                self.favorites.remove(i);
                false
            }
            None => {
                self.favorites.push(CartKey::of(cart));
                true
            }
        }
    }

    pub fn write_favorites(&self, root: &Path) -> io::Result<()> {
        write_list(root, FAVORITES_FILE, &self.favorites)
    }

    pub fn favorite_carts<'a>(&self, carts: impl Iterator<Item = &'a Cart>) -> Vec<Cart> {
        let mut out: Vec<Cart> = carts.filter(|c| self.is_favorite(c)).cloned().collect();
        out.sort_by_key(|c| (sort_key(&c.stem), c.platform as u8));
        out
    }
}

pub fn import_legacy_recents(root: &Path, state: &mut SlotState) -> io::Result<bool> {
    if !root.join(CONFIG_DIR).join(RECENTS_FILE).exists() {
        return Ok(false);
    }
    if state.recents.is_empty() {
        let old = read_list(root, RECENTS_FILE)?;
        state.show_recents |= !old.is_empty();
        state.recents = old
            .into_iter()
            .take(RECENTS_MAX)
            .map(|k| Recent {
                platform: k.platform,
                stem: k.stem,
            })
            .collect();
    }
    Ok(true)
}

pub fn remove_legacy_recents(root: &Path) -> io::Result<()> {
    std::fs::remove_file(root.join(CONFIG_DIR).join(RECENTS_FILE))
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

fn write_list(root: &Path, file: &str, keys: &[CartKey]) -> io::Result<()> {
    let dir = root.join(CONFIG_DIR);
    std::fs::create_dir_all(&dir)?;
    let text: String = keys.iter().map(CartKey::line).collect();
    atomic_write(&dir.join(file), text.as_bytes())
}
