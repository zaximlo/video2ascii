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
    
    let mut img_pxls: Vec<[f32; 3]> = Vec::new();

    let mut W: i32 = 0;

    videoinput::vectorize("main.mp4", 1, &mut img_pxls, &mut W);

    let H: i32 = img_pxls.len() as i32 / W;

    println!("{} {}", W, H);

    // println!("{:?}", img_pxls);

    let black = Color::new(0.0, 0.0, 0.0, 1.);

    loop {
        clear_background(black);

        let mut index: usize = 0;

        for y in 0..H {
            for x in 0..W {
                let pixel = img_pxls[index];

                let to_col = Color::new(pixel[0], pixel[1], 0.4, 1.);

                draw_rectangle(x as f32, y as f32, 1.0, 1.0, to_col);

                index += 1;

                // println!("{:?}", pixel);

                
                // println!("d");


            }
        }

        let mut img_pxls: Vec<[f32; 3]> = Vec::new();
        videoinput::vectorize("main.mp4", 290, &mut img_pxls, &mut W);


        next_frame().await


    }

    for _ in 0..44 {println!("hjhfjhghu---------------------------------------");}
}