extern crate sqlite;

use error::OtakuError;
use std::path::PathBuf;

pub struct Catalogue {
    connection: sqlite::Connection,
}

pub fn open(catalogue_db_path: PathBuf) -> Result<Catalogue, OtakuError> {
    let catalogue = sqlite::open(catalogue_db_path)?;
    Ok(Catalogue {
        connection: catalogue,
    })
}


pub fn bootstrap(catalogue_db: Catalogue) -> Result<(), OtakuError> {
    catalogue_db
        .connection
        .execute(include_str!("bootstrap/bootstrap.sql"))?;
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

        let result = sqlite.is_ok();
        fs::remove_file(&test_db_file).unwrap();

        assert!(result == true);
    }

    #[test]
    fn test_we_can_bootstrap_the_db() {
        let test_db_file = Path::join(&env::temp_dir(), "test2.sqlite");
        let sqlite = open(PathBuf::from(&test_db_file)).unwrap();

        let result = bootstrap(sqlite).is_ok();
        fs::remove_file(&test_db_file).unwrap();

        assert!(result == true);
    }
}
