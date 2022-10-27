mod videoinput;

use opencv::prelude::*;
use opencv::Result;

use std::thread;
use std::time::*;

use rgb::RGB8;

use macroquad::prelude::*;
use macroquad::color::Color;

/// Encode video into ascii animation
#[macroquad::main("idk")]
async fn main(){
    
    let mut img_pxls: Vec<[u8; 3]> = Vec::new();

    videoinput::vectorize("main.mp4", &mut img_pxls);

    println!("{:?}", img_pxls[5]);

    let black = Color::new(0.0, 0.0, 0.0, 1.);

    if 1==1 {
        clear_background(black);
        next_frame().await

    }

    for _ in 0..44 {println!("hjhfjhghu---------------------------------------");}
}