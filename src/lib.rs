use colored::Colorize;
use image::imageops::FilterType;
use image::codecs::gif::GifDecoder;

use image::codecs::webp::WebPDecoder;
use image::{
    AnimationDecoder, DynamicImage, GenericImage, GenericImageView, ImageBuffer,
    ImageFormat, Pixel, Primitive, Rgba
};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::io;

type Image = ImageBuffer<Rgba<u8>, Vec<u8>>;

#[allow(unused_comparisons)]
pub fn run() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && args[1] == "help" {
        display_function_definitions();
    } else {
        let functions = [
            "asciiart",
            "concatenate",
            "convert",
            "scaledown",
            "scaleup",
            "pixelate",
            "settransparency",
            "grayscale",
            "extractwebp",
            "extractgif",
            "flipv",
            "fliph",
            "rotate90",
            "rotate180",
            "rotate270"
        ];

        for (n, name) in functions.iter().enumerate() {
            println!("{}: {}", n, name);
        }

        println!("Enter option num: ");

        let function_num = match read_option() {
            Ok(num) => num,
            Err(error) => {
                eprintln!("Error reading option: {}", error);
                return;
            }
        };

        if function_num < 0 || function_num >= functions.len() {
            eprintln!("Invalid option number.");
            return;
        }

        let function_name = functions[function_num];

        match run_function(function_name) {
            Ok(_) => println!("Function {} called successfully.", function_name),
            Err(error) => eprintln!("Error calling function {}: {}", function_name, error),
        }
    }
}

fn display_function_definitions() {
    // println!("Available Functions:");
    // println!("0: AsciiArt");
    // println!("1: Concatenate");
    // println!("2: Convert");
    // println!("3: Pixelate");
    // println!("4: Colorfilter");
    // println!("5: SetTransparency");
    // println!("6: Scaleup");
    // println!("7: Scaledown");
    // println!("8: Grayscale");
    // println!("9: Extract");
}

fn read_option() -> Result<usize, io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let trimmed = input.trim();
    let option_num = trimmed.parse();
    match option_num {
        Ok(num) => Ok(num),
        Err(_) => Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid option")),
    }
}

fn arg() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(error) => println!("error: {error}"),
    }
    return input.replace("\r", "").replace("\n", "");
}

fn get_output() -> String{
    println!("Enter output file path: ");
    let output = arg();
    if Path::new(&output).exists() {
        println!("File already exists. Do you want to overwrite it? (y/n): ");
        let overwrite = arg();
        if overwrite != "y" {
            println!("Choose a different output file name.");
        }
    }
    return output
}

fn run_function(function_name: &str) -> Result<(), String> {
    println!("Enter file path: ");
    let path = arg();

    let img = match image::open(&path) {
        Ok(image) => image,
        Err(_) => {
            eprintln!("Error: The input file does not exist or is not a valid image.");
            return Err("File not found or invalid image".to_string());
        }
    };
    
    let (x, y) = img.dimensions();
    println!(
        "\nDetail: \nwidth: {} \nheight: {} \nformat: {:?}\n",
        x,
        y,
        ImageFormat::from_path(&path).unwrap()
    );
    
    match function_name.to_ascii_lowercase().as_str() {
        "asciiart" => {
            let output = get_output();
            ascii_art(&img, x, y, 20, &output);
            Ok(())
        }
        "convert" => {
            let output = get_output();
            let mut format_input = output.split(".");
            let format = match format_input.nth(1).unwrap() {
                "avif" => ImageFormat::Avif,
                "jpg" | "jpeg" => ImageFormat::Jpeg,
                "png" => ImageFormat::Png,
                "gif" => ImageFormat::Gif,
                "webp" => ImageFormat::WebP,
                "tif" | "tiff" => ImageFormat::Tiff,
                "tga" => ImageFormat::Tga,
                "dds" => ImageFormat::Dds,
                "bmp" => ImageFormat::Bmp,
                "ico" => ImageFormat::Ico,
                "hdr" => ImageFormat::Hdr,
                "exr" => ImageFormat::OpenExr,
                "pbm" | "pam" | "ppm" | "pgm" => ImageFormat::Pnm,
                "ff" | "farbfeld" => ImageFormat::Farbfeld,
                "qoi" => ImageFormat::Qoi,
                _ => return Err("Format not exist".to_string()),
            };
            let _ = img.save_with_format(output, format);
            Ok(())
        }
        "pixelate" => {
            println!("Enter output text file: ");
            let output = get_output();
            println!("Enter numbers of pixel: ");
            let pixel: u32 = arg().parse().unwrap();
            let img = pixelate(&img, (x / pixel, y / pixel));
            let _ = img.save(&output);
            Ok(())
        }
        "settransparency" => {
            let output = get_output();
            let img = transparent(&img);
            let _ = img.save(&output);
            Ok(())
        }
        "concatenate" => {
            let output = get_output();
            let mut v: Vec<DynamicImage> = vec![img];
            loop {
                println!("Enter another image or s to start the process: ");
                let more = arg();
                if more == "s" {
                    break;
                }
                v.append(&mut vec![image::open(more).unwrap()])
            }
            println!("Enter h for horizontal concatenation or v for vertical concatenation: ");
            let check = arg();
            let img = concat(v.as_slice(), &check);
            let _ = img.save(&output);
            Ok(())
        }
        "grayscale" => {
            let output = get_output();
            let img = grayscale(&img);
            let _ = img.save(&output);
            Ok(())
        }
        "extractwebp" => {
            extractwebp(&path);
            Ok(())
        } 
        "extractgif" => {
            extractwebp(&path);
            Ok(())
        } 
        "flipv" => {
            let output = get_output();
            let img = flipv(&img);
            let _ = img.save(&output);
            Ok(())
        }
        "fliph" => {
            let output = get_output();
            let img = fliph(&img);
            let _ = img.save(&output);
            Ok(())
        }
        "rotate90" => {
            let output = get_output();
            let img = rotate90(&img);
            let _ = img.save(&output);
            Ok(())
        }
        "rotate180" => {
            let output = get_output();
            let img = rotate180(&img);
            let _ = img.save(&output);
            Ok(())
        }
        "rotate270" => {
            let output = get_output();
            let img = rotate270(&img);
            let _ = img.save(&output);
            Ok(())
        }
        "scaledown" => {
            let output = get_output();
            println!("Enter new scale: ");
            let scale = arg().parse().unwrap();
            let img = img.resize(scale, scale,FilterType::Lanczos3);
            let _ = img.save(&output);
            Ok(())
        }
        "scaleup" => {
            let output = get_output();
            println!("Enter scale: ");
            let scale = arg().parse().unwrap();
            let img = img.resize(scale, scale,FilterType::Nearest);
            let _ = img.save(&output);
            Ok(())
        }
        "resize" => {
            let output = get_output();
            println!("Enter new width: ");
            let w = arg().parse().unwrap();
            println!("Enter new height: ");
            let h = arg().parse().unwrap();
            let img = res(&img.to_rgba8(), (w, h));
            let _ = img.save(&output);
            Ok(())
        }
        _ => Err(format!("Function {} not working properly.", function_name)),
    }
}

pub fn ascii_art(img: &DynamicImage, width: u32, height: u32, scale: u32, output: &str) {
    let mut file = File::create(output).expect("Failed");
    let mut text = String::new();
    for y in 0..height {
        for x in 0..width {
            if y % (scale * 2) == 0 && x % scale == 0 {
                let pix = img.get_pixel(x, y);
                let mut intent = pix[0] / 3 + pix[1] / 3 + pix[2] / 3;
                if pix[3] == 0 {
                    intent = 0;
                }
                print!(
                    "{}",
                    get_str_ascii(intent).truecolor(pix[0], pix[1], pix[2])
                );
                text += get_str_ascii(intent);
            }
        }
        if y % (scale * 2) == 0 {
            println!("");
            text += "\n"
        }
    }
    let _ = file.write_all(text.as_bytes()).expect("write failed");
}

pub fn get_str_ascii(intent: u8) -> &'static str {
    let mut ascii = [
        "Ñ", "@", "#", "W", "$", "9", "8", "7", "6", "5", "4", "3", "2", "1", "0", "?", "!", "a",
        "b", "c", ";", ":", "+", "=", "-", ",", ".", "_", " ",
    ];
    let scale: u8 = (255. / ascii.len() as f32).ceil() as u8;
    let index = intent / scale;
    ascii.reverse();
    return ascii[index as usize];
}

fn res(img: &Image, new_dims: (u32, u32)) -> Image {
    let (old_width, old_height) = img.dimensions();
    let (new_width, new_height) = new_dims;

    let mut resized = ImageBuffer::new(new_width, new_height);

    for (new_x, new_y, pixel) in resized.enumerate_pixels_mut() {
        let old_x = (new_x as f32 * (old_width as f32 / new_width as f32)) as u32;
        let old_y = (new_y as f32 * (old_height as f32 / new_height as f32)) as u32;

        if let Some(old_pixel) = img.get_pixel_checked(old_x, old_y) {
            *pixel = *old_pixel;
        } else {
            println!("({old_x} -> {new_x}, {old_y} -> {new_y})");
        }
    }

    resized
}

fn pixelate(img: &DynamicImage, new_dims: (u32, u32)) -> Image {
    let old_dims = img.dimensions();
    let img = img.to_rgba8();
    let small = res(&img, ((old_dims.0 / new_dims.0), (old_dims.1 / new_dims.1)));
    let pixelated = res(&small, old_dims);

    pixelated
}

pub fn transparent(img: &DynamicImage) -> Image {
    println!("Enter transparency percentage: ");
    let value: f32 = (100. - arg().parse::<f32>().unwrap()) / 100. * 255.;
    let mut img = img.to_rgba8();

    for (_x, _y, pixel) in img.enumerate_pixels_mut() {
        *pixel = image::Rgba([pixel[0], pixel[1], pixel[2], value as u8]);
    }

    img
}

pub fn fadetop(img: &DynamicImage, width: u32) -> Image {
    let mut img = img.to_rgba8();

    for (x, _y, pixel) in img.enumerate_pixels_mut() {
        let tsp = 255. - x as f32 / width as f32 * 255.;
        *pixel = image::Rgba([pixel[0], pixel[1], pixel[2], tsp as u8]);
    }

    img
}

pub fn concat<I, P, S>(images: &[I], check: &str) -> ImageBuffer<P, Vec<S>>
where
    I: GenericImageView<Pixel = P>,
    P: Pixel<Subpixel = S> + 'static,
    S: Primitive + 'static,
{
    let (img_width_out, img_height_out) = match check {
        "h" => (
            images.iter().map(|im| im.width()).sum(),
            images.iter().map(|im| im.height()).max().unwrap_or(0),
        ),
        "v" => (
            images.iter().map(|im| im.width()).max().unwrap_or(0),
            images.iter().map(|im| im.height()).sum(),
        ),
        _ => (0, 0),
    };

    let mut imgbuf = image::ImageBuffer::new(img_width_out, img_height_out);

    let mut accumulated_value = 0;

    for img in images {
        match check {
            "h" => {
                imgbuf.copy_from(img, accumulated_value, 0).unwrap();
                accumulated_value += img.width();
            }
            "v" => {
                imgbuf.copy_from(img, 0, accumulated_value).unwrap();
                accumulated_value += img.height();
            }
            _ => todo!(),
        }
    }

    imgbuf
}

pub fn grayscale(img: &DynamicImage) -> DynamicImage {
    let img = img.grayscale();
    img
}

pub fn extractwebp(path: &str) {
    let filename = path.split(".").nth(0).unwrap();
    let file_in = File::open(&path).unwrap();
    let decoder = WebPDecoder::new(file_in).unwrap();
    let frames = decoder.into_frames();
    let frames = frames.collect_frames().expect("error decoding");
    let _= fs::create_dir_all(filename);
    frames.iter().enumerate().for_each(|(i, f)| f.buffer().save(format!(r"{filename}\frame{i}.png")).unwrap());
}

pub fn extractgif(path: &str) {
    let filename = path.split(".").nth(0).unwrap();
    let file_in = File::open(&path).unwrap();
    let decoder = GifDecoder::new(file_in).unwrap();
    let frames = decoder.into_frames();
    let frames = frames.collect_frames().expect("error decoding");
    let _= fs::create_dir_all(filename);
    frames.iter().enumerate().for_each(|(i, f)| f.buffer().save(format!(r"{filename}\frame{i}.png")).unwrap());
}

pub fn fliph(image: &DynamicImage) -> Image{
    let (width, height) = image.dimensions();
    let mut img: Image = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width / 2 {
            let x2 = width - x - 1;
            let p2 = image.get_pixel(x2, y);
            let p = image.get_pixel(x, y);
            img.put_pixel(x2, y, p);
            img.put_pixel(x, y, p2);
        }
    }
    img
}

pub fn flipv(image: &DynamicImage) -> Image {
    let (width, height) = image.dimensions();
    let mut img: Image = ImageBuffer::new(width, height);

    for y in 0..height / 2 {
        for x in 0..width {
            let y2 = height - y - 1;
            let p2 = image.get_pixel(x, y2);
            let p = image.get_pixel(x, y);
            img.put_pixel(x, y2, p);
            img.put_pixel(x, y, p2);
        }
    }
    img
}

pub fn rotate90(image: &DynamicImage) -> Image{
    let (width, height) = image.dimensions();
    let mut img: Image = ImageBuffer::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let p = image.get_pixel(x, y);
            img.put_pixel(height - y - 1, x, p);
        }
    }
    img
}

pub fn rotate180(image: &DynamicImage) -> Image{
    let (width, height) = image.dimensions();
    let mut img: Image = ImageBuffer::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let p = image.get_pixel(x, y);
            img.put_pixel(width - x - 1, height - y - 1, p);
        }
    }
    img
}

pub fn rotate270(image: &DynamicImage) -> Image{
    let (width, height) = image.dimensions();
    let mut img: Image = ImageBuffer::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let p = image.get_pixel(x, y);
            img.put_pixel(y, width - x - 1, p);
        }
    }
    img
}