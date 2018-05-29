extern crate sqlite;

use error::OtakuError;
use std::path::PathBuf;

pub fn open(catalogue_db_path: PathBuf) -> Result<sqlite::Connection, OtakuError> {
    let catalogue = sqlite::open(catalogue_db_path)?;
    Ok(catalogue)
}

pub fn bootstrap(catalogue_db: sqlite::Connection) -> Result<(), OtakuError> {
    catalogue_db.execute(include_str!("bootstrap/bootstrap.sql"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use std::path::{Path, PathBuf};

    #[test]
    fn test_we_can_open_the_db() {
        let test_db_file = Path::join(&env::temp_dir(), "test.sqlite");
        let sqlite = open(PathBuf::from(&test_db_file));

        assert!(sqlite.is_ok());

        fs::remove_file(&test_db_file).unwrap();
    }

    #[test]
    fn test_we_can_bootstrap_the_db() {
        let test_db_file = Path::join(&env::temp_dir(), "test2.sqlite");
        let sqlite = open(PathBuf::from(&test_db_file)).unwrap();

        assert!(bootstrap(sqlite).is_ok());

        fs::remove_file(&test_db_file).unwrap();
    }
}
