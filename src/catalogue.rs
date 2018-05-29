use std::path::PathBuf;
use error::OtakuError;

pub fn open(catalogue_db_path: PathBuf) -> Result<String, OtakuError> {
    let str = String::from("w00t");
    Ok(str)
}

pub fn bootstrap(catalogue_db: String) -> Result<i32, OtakuError> {
    Ok(32)
}
