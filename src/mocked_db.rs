use crate::db::{Db, Entry};
use anyhow::Result;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::fs::File;

#[derive(Clone)]
pub struct MockedDb {
    folder: PathBuf,
}

impl MockedDb {
    pub fn new(folder: &str) -> Self {
        let folder = std::env::temp_dir().join(folder);
        println!("folder {} will be used as a mock storage", folder.to_string_lossy());
        std::fs::create_dir_all(&folder).expect("Cannot create mock storage");
        Self {
            folder
        }
    }

    fn get_new_key(&self) -> String {
        use rand::{Rng, distributions::Alphanumeric};

        loop {
            let key: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(8)
                .map(char::from)
                .collect();

            if !(&self.folder.join(&key).exists()) {
                break key;
            }
        }
    }
}

impl Db for MockedDb {
    fn add(&self, long: &str) -> Result<Entry> {
        let short = self.get_new_key();

        let entry = Entry::new(short.into(), long.into());

        let mut f = File::create(self.folder.join(entry.short.clone()))?;
        f.write_all(entry.long.as_bytes())?;
        println!("data saved into {:?}", f);

        Ok(entry)
    }

    fn get(&self, short: &str) -> Result<Entry> {
        let file_path = Path::new(&self.folder).join(short);
        let long = std::fs::read_to_string(file_path)?;

        Ok(Entry::new(short.into(), long))
    }
}
