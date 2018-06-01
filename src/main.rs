extern crate base64;
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

    let catalogue = catalogue::open(catalogue_db_path)
        .expect("could not open catalogue db");

    if !catalogue.is_bootstrapped().is_ok() {
        catalogue::bootstrap(&catalogue)
            .expect("could not bootstrap catalogue db");
    }

    downloader::download_image(
        "https://www.google.com/images/branding/googlelogo/1x/googlelogo_color_272x92dp.png",
        &catalogue_data_path,
        &catalogue
    ).expect("could not download image");
}
