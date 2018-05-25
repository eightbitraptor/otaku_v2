extern crate reqwest;

use base64::encode;
use std::fs;

pub fn download_image(url: &str, download_dir: &str) -> String {
    let image_name = encode(url);
    let download_name = format!("{}/{}", download_dir, image_name);
    let mut image = reqwest::get(url).expect("unable_to_download_file");
    let mut buffer: Vec<u8> = vec![];

    image
        .copy_to(&mut buffer)
        .expect("unable to write response to buffer");

    fs::write(&download_name, buffer).expect("unable to write file");

    return download_name;
}

#[cfg(test)]
mod tests {
    extern crate image;

    use super::*;
    use std::path::Path;

    #[test]
    fn test_it_downloads_an_image_from_a_url() {
        let img_url =
            "https://www.google.com/images/branding/googlelogo/1x/googlelogo_color_272x92dp.png";

        let directory = "/tmp";
        let img_file_name = download_image(&img_url, &directory);

        assert!(Path::new(&img_file_name).exists());
    }

    #[test]
    fn test_it_can_open_the_saved_image() {
        use std::fs::File;
        use std::io::prelude::*;

        let img_url =
            "https://www.google.com/images/branding/googlelogo/1x/googlelogo_color_272x92dp.png";
        let directory = "/tmp";

        let img_file_name = download_image(&img_url, &directory);

        let mut f = File::open(&img_file_name).unwrap();
        let mut buffer = [0; 10];
        f.read_exact(&mut buffer)
            .expect("Failed to read bytes into test buffer");

        assert_eq!(
            image::guess_format(&buffer).expect("failed to read buffer"),
            image::ImageFormat::PNG
        );
    }
}
