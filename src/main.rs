extern crate base64;
extern crate xdg;

mod downloader;

fn main() {
    let base_dirs = xdg::BaseDirectories::with_prefix("otaku")
        .expect("could not find XDG Base Directory settings");

    let catalogue_db_path = base_dirs
        .place_data_file("catalogue.sqlite")
        .expect("could not get data path");

    downloader::download_image(
        "https://www.google.com/images/branding/googlelogo/1x/googlelogo_color_272x92dp.png",
        "/tmp",
    );
}
