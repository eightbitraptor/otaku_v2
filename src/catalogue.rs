use downloader;
use error::*;
use sqlite;
use sqlite::{Connection, State};
use std::path::PathBuf;

pub struct Catalogue<'a> {
    pub data_path: &'a PathBuf,
    conn: Connection,
}

pub fn open(catalogue_db_path: &PathBuf) -> Result<Catalogue> {
    let conn = sqlite::open(catalogue_db_path)?;

    let catalogue = Catalogue {
        data_path: catalogue_db_path,
        conn: conn,
    };
    Ok(catalogue)
}

pub fn bootstrap(cat: &Catalogue) -> Result<()> {
    cat.conn.execute(include_str!("bootstrap/bootstrap.sql"))?;
    Ok(())
}

pub fn db_state(cat: &Catalogue) -> Result<()> {
    let mut statement = cat
        .conn
        .prepare(include_str!("bootstrap/check_bootstrap.sql"))?;

    let value = statement.next().and_then(|_| statement.read::<i64>(0));

    match value {
        Ok(1) => Ok(()),
        _ => Err(OtakuError {}),
    }
}

pub fn image_to_catalogue(image_url: &str, cat: &Catalogue) -> Result<()> {
    downloader::fetch_image(image_url, cat.data_path)
        .and_then(|image| insert_image(&cat, &image, "2018-01-01"))?;
    Ok(())
}

fn insert_image(cat: &Catalogue, name: &str, created: &str) -> Result<()> {
    let mut statement = cat
        .conn
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
        let cat = open(&test_db_file);

        let result = cat.is_ok();
        fs::remove_file(&test_db_file).unwrap();

        assert!(result == true);
    }

    #[test]
    fn test_we_can_bootstrap_the_db() {
        let test_db_file = generate_db_filename();
        let cat = open(&test_db_file).unwrap();

        let result = bootstrap(&cat).is_ok();
        fs::remove_file(&test_db_file).unwrap();

        assert!(result == true);
    }

    #[test]
    fn test_db_state_when_db_db_state() {
        let test_db_file = generate_db_filename();
        let cat = open(&test_db_file).unwrap();
        bootstrap(&cat).expect("problems bootstrapping db");

        let result = db_state(&cat);
        fs::remove_file(&test_db_file).unwrap();

        assert!(result.is_ok());
    }

    #[test]
    fn test_db_state_when_db_is_not_bootstrapped() {
        let test_db_file = generate_db_filename();
        let cat = open(&test_db_file).unwrap();

        let result = db_state(&cat);
        fs::remove_file(&test_db_file).unwrap();

        assert!(result.is_err());
    }

    #[test]
    fn test_db_state_when_db_is_badly_bootstrapped() {
        let test_db_file = generate_db_filename();
        let cat = open(&test_db_file).unwrap();

        cat.conn
            .execute("CREATE TABLE schema_versions (id INT)")
            .unwrap();

        let result = db_state(&cat);
        fs::remove_file(&test_db_file).unwrap();

        assert!(result.is_err());
    }

    #[test]
    fn test_inserting_images_into_the_catalogue() {
        let test_db_file = generate_db_filename();
        let cat = open(&test_db_file).unwrap();

        bootstrap(&cat).unwrap();

        let result = insert_image(&cat, "my_image_name", "2018-01-01");
        fs::remove_file(&test_db_file).unwrap();

        assert!(result.is_ok());
    }
}
