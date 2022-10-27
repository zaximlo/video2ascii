use opencv::videoio::VideoCapture;
use opencv::videoio::CAP_ANY;
use opencv::Result;

use opencv::prelude::*;
use rgb::RGB8;


pub fn from_file(path: &str) -> Result<VideoCapture> {
    VideoCapture::from_file(path, CAP_ANY)
}

pub fn vectorize(filename: &str, thing: &mut Vec<[u8; 3]> ) -> Result<String>{
    let mut video = from_file("test.mp4")?;

    let mut times: u64 = 0;
    
    // Make img
    let mut img = Mat::default();
    video.read(&mut img).unwrap_or(false);

    let mut img_pxls: Vec<[u8; 3]> = Vec::new();

    let size: i32 = img.rows()*img.cols();

    for i in 0..img.rows() {
        for j in 0..img.cols() {
            let pixel = img.at_2d::<RGB8>(i, j).unwrap();
            
            img_pxls.push([pixel.r, pixel.g, pixel.b]);

        }
    }

    Ok("fart".to_string())
}