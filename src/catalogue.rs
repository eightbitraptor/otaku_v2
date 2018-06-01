use error::*;
use sqlite;
use sqlite::{Connection, State};
use std::path::PathBuf;

pub fn open(catalogue_db_path: PathBuf) -> Result<Connection> {
    let conn = sqlite::open(catalogue_db_path)?;
    Ok(conn)
}

pub fn bootstrap(conn: &Connection) -> Result<()> {
    conn.execute(include_str!("bootstrap/bootstrap.sql"))?;
    Ok(())
}

pub fn insert_image(conn: &Connection, name: &str, created: &str) -> Result<()> {
    let mut statement = conn
        .prepare(include_str!("queries/insert_image.sql"))
        .unwrap();

    statement.bind(1, name)?;
    statement.bind(2, created)?;

    match statement.next() {
        Ok(State::Done) => return Ok(()),
        Ok(_) => return Err(OtakuError {}),
        Err(e) => return Err(OtakuError::from(e)),
    }
}

pub fn is_bootstrapped(conn: &Connection) -> Result<()> {
    let mut statement = conn.prepare(include_str!("bootstrap/check_bootstrap.sql"))?;

    let value = statement.next().and_then(|_| statement.read::<i64>(0));

    match value {
        Ok(1) => Ok(()),
        _ => Err(OtakuError {}),
    }
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

        let result = is_bootstrapped(&sqlite);
        fs::remove_file(&test_db_file).unwrap();

        assert!(result.is_ok());
    }

    #[test]
    fn test_is_bootstrapped_when_db_is_not_bootstrapped() {
        let test_db_file = generate_db_filename();
        let sqlite = open(PathBuf::from(&test_db_file)).unwrap();

        let result = is_bootstrapped(&sqlite);
        fs::remove_file(&test_db_file).unwrap();

        assert!(result.is_err());
    }

    #[test]
    fn test_is_bootstrapped_when_db_is_badly_bootstrapped() {
        let test_db_file = generate_db_filename();
        let sqlite = open(PathBuf::from(&test_db_file)).unwrap();

        sqlite
            .execute("CREATE TABLE schema_versions (id INT)")
            .unwrap();

        let result = is_bootstrapped(&sqlite);
        fs::remove_file(&test_db_file).unwrap();

        assert!(result.is_err());
    }

    #[test]
    fn test_inserting_images_into_the_catalogue() {
        let test_db_file = generate_db_filename();
        let sqlite = open(PathBuf::from(&test_db_file)).unwrap();

        bootstrap(&sqlite).unwrap();

        let result = insert_image(&sqlite, "my_image_name", "2018-01-01");
        fs::remove_file(&test_db_file).unwrap();

        assert!(result.is_ok());
    }
}
