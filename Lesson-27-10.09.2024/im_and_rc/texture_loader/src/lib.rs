use std::{collections::HashMap, path::PathBuf, rc::{Rc, Weak}};

#[derive(Default)]
pub struct TextureLoader {
    cache: HashMap<PathBuf, Weak<Texture>>,
}

impl TextureLoader {
    pub fn load(&mut self, path: PathBuf) -> Option<Rc<Texture>> {
        use std::collections::hash_map::Entry;

        match self.cache.entry(path.clone()) {
            Entry::Occupied(mut occupied) => {
                let weak = occupied.get();
                if let Some(strong) = weak.upgrade() {
                    Some(strong)
                } else {
                    let texture = Self::load_from_disk(path)?;
                    let strong = Rc::new(texture);
                    occupied.insert(Rc::downgrade(&strong));
                    Some(strong)
                }
            },
            Entry::Vacant(vacant) => {
                let texture = Self::load_from_disk(path)?;
                let strong = Rc::new(texture);
                vacant.insert(Rc::downgrade(&strong));
                Some(strong)
            },
        }
    }

    fn load_from_disk(_path: PathBuf) -> Option<Texture> {
        Some(Texture)
    }
}

pub struct Texture;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn texture_loader_works() {
        let mut loader = TextureLoader::default();

        let texture_opt = loader.load(PathBuf::from("test")).unwrap();
        assert_eq!(Rc::strong_count(&texture_opt), 1);

        let texture_opt = loader.load(PathBuf::from("test")).unwrap();
        assert_eq!(Rc::strong_count(&texture_opt), 2);
    }
}
