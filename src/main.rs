/*
Ahmed - A rust framework created to do some math for "project xy" visualation
but it can be used to animate anything! except images and polygons and text....
*/

use image::{Rgb, RgbImage, ImageBuffer};
use std::path::Path;
use std::f32::consts::PI;

fn draw_rectangle(img: &mut RgbImage, x: u32, y: u32, width: u32, height: u32, color: Rgb<u8>) {
    for i in x..x+width {
        for j in y..y+height {
            img.put_pixel(i, j, color);
        }
    }
}

fn draw_circle(img: &mut RgbImage, center_x: i32, center_y: i32, radius: i32, color: Rgb<u8>) {
    for x in (center_x - radius)..(center_x + radius) {
        for y in (center_y - radius)..(center_y + radius) {
            let distance = ((x - center_x) as f64).powi(2) + ((y - center_y) as f64).powi(2);
            if distance.sqrt() <= radius as f64 {
                img.put_pixel(x as u32, y as u32, color);
            }
        }
    }
}

fn draw_triangle(img: &mut RgbImage, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, color: Rgb<u8>) {
    draw_line(img, x1, y1, x2, y2, color);
    draw_line(img, x2, y2, x3, y3, color);
    draw_line(img, x3, y3, x1, y1, color);
}

fn draw_line(img: &mut RgbImage, x1: i32, y1: i32, x2: i32, y2: i32, color: Rgb<u8>) {
    let mut x = x1;
    let mut y = y1;
    let dx = (x2 - x1).abs();
    let dy = (y2 - y1).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = if dx > dy { dx } else { -dy } / 2;

    loop {
        img.put_pixel(x as u32, y as u32, color);
        if x == x2 && y == y2 {
            break;
        }
        let e2 = err;
        if e2 > -dx {
            err -= dy;
            x += sx;
        }
        if e2 < dy {
            err += dx;
            y += sy;
        }
    }
}

fn main() {
    // Create a new image with a white background
    let frame: u32 = 200;
    let center_x = 400;
    let center_y = 300;
    let radius = 200;
    let mut angle: f32 = 2.0*PI+1.1;
    let mut a = 0;
    let mut img: Vec<ImageBuffer<Rgb<u8>, Vec<u8>>> = vec![RgbImage::new(800, 600)];

    for i in 0..frame {
        img.push(RgbImage::new(800, 600));
    }

    for i in 0..frame {
        let x = center_x as f32 + radius as f32 * angle.cos();
        let y = center_y as f32 + radius as f32 * angle.sin();
        draw_line(&mut img[a], center_x, center_y, (x as u32).try_into().unwrap(), (y as u32).try_into().unwrap(), Rgb([255, 255, 255]));
        draw_circle(&mut img[a], (x as u32).try_into().unwrap(), (y as u32).try_into().unwrap(), 25, Rgb([255, 0, 0]));
        println!("{}", a);

        let file_name = format!("img{}.png", i);
        let path = Path::new(&file_name);
        let file_path = path.to_str().unwrap();
        img[a].save(file_path).unwrap();
        angle += 0.1;
        a += 1;
    }
}
