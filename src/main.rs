// TODO - implement Complex myself for practice
use image::codecs::png::PngEncoder;
use image::{ExtendedColorType, ImageEncoder};
use num::Complex;
use std::env;
use std::fs::File;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT", args[0]);
        eprintln!("Example: {} mandel.png, 1000x750, -1.20,0.35 -1,0.20", args[0]);
        std::process::exit(1);
    }
    let bounds = parse_pair(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("error parsing upper left corner point");
    let lower_right = parse_complex(&args[4]).expect("error parsing lower_right corner point");
    let mut pixels = vec![0; bounds.0 * bounds.1];

    render(&mut pixels, bounds, upper_left, lower_right);
    write_image(&args[1], &pixels, bounds).expect("error writing PNG file");
}

fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> image::error::ImageResult<()> {
    let output = File::create(filename)?;
    let encoder = PngEncoder::new(output);
    let _ = encoder.write_image(pixels, bounds.0 as u32, bounds.1 as u32, ExtendedColorType::L8);
    Ok(())
}

/// Render a rectangle of the fractal into a buffer of pixels
///
/// The `bounds` argument gives the width and height of the buffer in `pixels`, which holds one grayscale pixel per byte.
/// The `upper_left` and `lower_right` arguments specify points on the complex plane which corresponde to
/// the upper-left and lower-right corners of the pixel buffer.
fn render(pixels: &mut [u8], bounds: (usize, usize), upper_left: Complex<f64>, lower_right: Complex<f64>) {
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for col in 0..bounds.0 {
            let point = pixel_to_point(bounds, (col, row), upper_left, lower_right);
            pixels[row * bounds.0 + col] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            };
        }
    }
}

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        // z diverges if its radius is greater than 2
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c
    }
    None
}

/// Given the row and column of a  pixel in the output image, return the corresponding point on the complex plane
///
/// `bounds` is a pair giving the width and height of the image in pixels
/// `pixel` is a (column, row) pair indicating a particular pixel in that image
/// The `upper_left` and `lower_right` parameters are points on the complex plane designating the area our image covers
fn pixel_to_point(
    bounds: (usize, usize), pixel: (usize, usize), upper_left: Complex<f64>, lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (lower_right.re - upper_left.re, upper_left.im - lower_right.im);

    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im + pixel.1 as f64 * height / bounds.1 as f64,
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point(
            (100, 200),
            (25, 175),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex { re: -0.5, im: -0.75 }
    )
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        _ => None,
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("-1.2,0.1"), Some(Complex { re: -1.2, im: 0.1 }));
    assert_eq!(parse_complex("-1.234,dklfjd"), None);
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20noise", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.2", 'x'), Some((0.5, 1.2)));
}
