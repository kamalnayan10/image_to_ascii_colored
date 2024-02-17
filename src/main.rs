use image::{GenericImageView, Rgba};
use colored::Colorize;
use std::cmp;
use crossterm::terminal;

fn get_ascii(color_avg:u8) -> &'static str{
    let idx = color_avg/52;

    let ascii_char = ["." , "," , ":" , "%" , "@"];

    return ascii_char[idx as usize]
}

fn main() {

    let (term_w , term_h) = terminal::size().unwrap();
    
    let path = "logo.png";
    let img = image::open(path).unwrap();

    let resized_img = img.resize((term_w) as u32, (term_h) as u32,image::imageops::FilterType::Nearest);
    
    let (width , height) = resized_img.dimensions();

    for y in 0..height{
        for x in 0..width{
            let pixels = resized_img.get_pixel(x,y);

            let mut color_avg: u8 = pixels[0]/3 + pixels[1]/3 + pixels[2]/3;

            if pixels[3] == 0{
                color_avg = 0;
            }

            let ascii = get_ascii(color_avg);

            
            print!("{:^3}" , ascii.truecolor(pixels[0],pixels[1],pixels[2]));
        }
        println!("");
    }
    

}