extern crate base64;
extern crate reqwest;
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

    let cat = catalogue::open(&catalogue_db_path).expect("could not open catalogue db");

    if !catalogue::db_state(&cat).is_ok() {
        catalogue::bootstrap(&cat).expect("could not bootstrap catalogue db");
    }

    let image =
        "https://www.google.com/images/branding/googlelogo/1x/googlelogo_color_272x92dp.png";
    catalogue::image_to_catalogue(&image, &cat)
        .expect("Image could not be downloaded");
}
