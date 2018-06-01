extern crate base64;
extern crate sqlite;
extern crate xdg;

mod catalogue;
mod downloader;
mod error;

fn main() {
    let base_dirs = xdg::BaseDirectories::with_prefix("otaku")
        .expect("could not find XDG Base Directory settings");

    let catalogue_data_path = base_dirs
        .create_data_directory("catalogue")
        .expect("could not create data path");

    let catalogue_db_path = base_dirs
        .place_data_file("catalogue.sqlite")
        .expect("could not get data path");

    let cat_conn = catalogue::open(catalogue_db_path).expect("could not open catalogue db");

    if !catalogue::db_state(&cat_conn).is_ok() {
        catalogue::bootstrap(&cat_conn).expect("could not bootstrap catalogue db");
    }

    let image = "https://www.google.com/images/branding/googlelogo/1x/googlelogo_color_272x92dp.png";
    downloader::fetch_image(image, &catalogue_data_path)
        .and_then(|image| catalogue::insert_image(&cat_conn, &image, "2018-01-01"))
        .expect("could not download image");
}
