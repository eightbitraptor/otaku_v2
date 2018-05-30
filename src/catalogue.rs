extern crate sqlite;

use self::sqlite::State;
use error::OtakuError;
use std::path::PathBuf;

pub struct Catalogue {
    connection: sqlite::Connection,
}

impl Catalogue {
    pub fn insert_image(&self, image_name: &str, created_date: &str) -> Result<i32, OtakuError> {
        let mut statement = self
            .connection
            .prepare(include_str!("queries/insert_image.sql"))
            .unwrap();

        statement.bind(1, image_name)?;
        statement.bind(2, created_date)?;

        match statement.next() {
            Ok(State::Done) => return Ok(0),
            Ok(_) => return Err(OtakuError{}),
            Err(e) => return Err(OtakuError::from(e))
        }
    }

    pub fn is_bootstrapped(&self) -> bool {
        let mut statement = match self
            .connection
            .prepare(include_str!("queries/latest_schema_version.sql"))
        {
            Ok(res) => res,
            Err(_) => return false,
        };

        let value = match statement.next().unwrap() {
            State::Row => statement.read::<i64>(0).unwrap(),
            State::Done => 0,
        };

        value > 0
    }
}

pub fn open(catalogue_db_path: PathBuf) -> Result<Catalogue, OtakuError> {
    let catalogue = sqlite::open(catalogue_db_path)?;
    Ok(Catalogue {
        connection: catalogue,
    })
}

pub fn bootstrap(catalogue_db: &Catalogue) -> Result<(), OtakuError> {
    catalogue_db
        .connection
        .execute(include_str!("bootstrap/bootstrap.sql"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    extern crate nanoid;

    use super::*;
    use std::env;
    use std::fs;
    use std::path::{Path, PathBuf};

    fn generate_db_filename() -> PathBuf {
        let generated_name = nanoid::simple();
        Path::join(&env::temp_dir(), format!("{}.sqlite", generated_name))
    }

    #[test]
    fn test_we_can_open_the_db() {
        let test_db_file = generate_db_filename();
        let sqlite = open(PathBuf::from(&test_db_file));

        let result = sqlite.is_ok();
        fs::remove_file(&test_db_file).unwrap();

        assert!(result == true);
    }

    #[test]
    fn test_we_can_bootstrap_the_db() {
        let test_db_file = generate_db_filename();
        let sqlite = open(PathBuf::from(&test_db_file)).unwrap();

        let result = bootstrap(&sqlite).is_ok();
        fs::remove_file(&test_db_file).unwrap();

        assert!(result == true);
    }

    #[test]
    fn test_is_bootstrapped_when_db_is_bootstrapped() {
        let test_db_file = generate_db_filename();
        let sqlite = open(PathBuf::from(&test_db_file)).unwrap();
        bootstrap(&sqlite).expect("problems bootstrapping db");

        let result = sqlite.is_bootstrapped();
        fs::remove_file(&test_db_file).unwrap();

        assert!(result == true);
    }

    #[test]
    fn test_is_bootstrapped_when_db_is_not_bootstrapped() {
        let test_db_file = generate_db_filename();
        let sqlite = open(PathBuf::from(&test_db_file)).unwrap();

        let result = sqlite.is_bootstrapped();
        fs::remove_file(&test_db_file).unwrap();

        assert!(result == false);
    }

    #[test]
    fn test_is_bootstrapped_when_db_is_badly_bootstrapped() {
        let test_db_file = generate_db_filename();
        let sqlite = open(PathBuf::from(&test_db_file)).unwrap();

        sqlite
            .connection
            .execute("CREATE TABLE schema_versions (id INT)")
            .unwrap();

        let result = sqlite.is_bootstrapped();
        fs::remove_file(&test_db_file).unwrap();

        assert!(result == false);
    }

    #[test]
    fn test_inserting_images_into_the_catalogue() {
        let test_db_file = generate_db_filename();
        let sqlite = open(PathBuf::from(&test_db_file)).unwrap();

        bootstrap(&sqlite).unwrap();

        let result = sqlite
            .insert_image("my_image_name", "2018-01-01");
        fs::remove_file(&test_db_file).unwrap();

        assert!(result.is_ok());
    }
}
