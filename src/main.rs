extern crate base64;

mod downloader;

fn main() {
    downloader::download_image(
        "https://www.google.com/images/branding/googlelogo/1x/googlelogo_color_272x92dp.png",
        "/tmp"
    );
}
