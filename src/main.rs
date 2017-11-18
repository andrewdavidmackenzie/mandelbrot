#![feature(test)]

extern crate num;
use num::Complex;

extern crate image;
use image::ColorType;
use image::png::PNGEncoder;

extern crate test;
extern crate dir_diff;
extern crate tempdir;

use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;
use std::io::Result;
use std::io::Write;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 5 {
        writeln!(std::io::stderr(), "Usage:   {} FILE PIXELS UPPERLEFT LOWERRIGHT", args[0]).unwrap();
        writeln!(std::io::stderr(), "Example: {} mandel.png 1000x750 -1.2,0.35 -1,0.20", args[0]).unwrap();
        std::process::exit(1);
    }

    let filename = PathBuf::from(&args[1]);
    let bounds = parse_pair(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse_pair(&args[3], ',').expect("error parsing upper left corner point");
    let lower_right = parse_pair(&args[4], ',').expect("error parsing lower right corner point");

    let mut pixels = vec![0; bounds.0 * bounds.1];

    render(&mut pixels, bounds, upper_left, lower_right);

    write_bitmap(&filename, &pixels, bounds).expect("error writing PNG file");
}

/// Parse the string 's' as a coordinate pair, like "400x600" or "1.0,0.5"
/// Specifically, 's' should have the form <left><sep><right> where <sep> is the character given by
/// the 'separator' argument, and <left> and <right> are both strings that can be parsed
/// by 'T::from_str'.
/// If 's' has the proper form, return 'Some<(x,y)>'.
/// If 's' doesn't parse correctly, return None.
fn parse_pair<T:FromStr>(s: &str, separator: char) -> Option<(T,T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index+1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}

/// Return the point on the complex plane corresponding to a given pixel in the bitmap
/// 'bounds' is a pair giving the width and height of the bitmap.
/// 'pixel' is a pair indicating a particular pixel in the bitmap.
/// 'upper_left' and 'upper_right' are points on the complex plane designating the area our
/// bitmap covers.
fn pixel_to_point(bounds: (usize, usize), pixel: (usize, usize),
                  upper_left: (f64, f64), lower_right: (f64, f64)) -> (f64, f64) {
    // it might be nicer to find the position on the middle of the pixel, instead of the
    // upper left corner, but this is easier to write tests for
    let (width, height) = (lower_right.0 - upper_left.0, upper_left.1 - lower_right.1);

    (upper_left.0 + pixel.0 as f64 * width  / bounds.0 as f64,
     upper_left.1 - pixel.1 as f64 * height / bounds.1 as f64)
}

/// Try to determine if 'c' is in the Mandlebrot set, using at most 'limit' iterations to decide
/// If 'c' is not a member, return 'Some(i)', where 'i' is the number of iterations it took for 'c'
/// to leave the circle of radius two centered on the origin.
/// If 'c' seems to be a member (more precisely, if we reached the iteration limit without being
/// able to prove that 'c' is not a member) return 'None'
fn escapes(c : Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex{ re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }

    return None;
}

/// Render a rectangle of the Mandlebrot set into a buffer of pixels
/// The 'bounds' argument gives the width and height of the buffer 'pixels' which holds one
/// grayscale pixel per byte. The 'upper_left' and 'lower_right' arguments specify points on the
/// complex plane corresponding to the upper left and lower right corners of the pixel buffer.
fn render(pixels: &mut [u8], bounds: (usize, usize),
          upper_left: (f64, f64), lower_right: (f64, f64)) {
    assert_eq!(pixels.len(), bounds.0 * bounds.1);

    for r in 0..bounds.1 {
        for c in 0..bounds.0 {
            let point = pixel_to_point(bounds, (c,r), upper_left, lower_right);
            // TODO if it reaches 255 we also write a zero, so make limit 254 to speed up?
            pixels[r * bounds.0 + c ] = match escapes(Complex {re: point.0, im: point.1}, 255) {
                None => 0, // TODO this write of a zero can be removed....
                Some(count) => 255 - count as u8
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use tempdir::TempDir;

    #[test]
    fn test_pixel_to_point() {
        assert_eq!(pixel_to_point((100, 100), (25,75), (-1.0, 1.0), (1.0, -1.0)),
                   (-0.5, -0.5));
    }

    #[test]
    fn test_parse_pair() {
        assert_eq!(parse_pair::<i32>("", ','), None);
        assert_eq!(parse_pair::<i32>("10,", ','), None);
        assert_eq!(parse_pair::<i32>(",10", ','), None);
        assert_eq!(parse_pair::<i32>("10,20", ','), Some((10,20)));
        assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
        assert_eq!(parse_pair::<f64>("0.5x", ','), None);
        assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
    }

    #[bench]
    fn bench_render_1000_by_1000(b: &mut Bencher) {
        let bounds = (1000, 1000);
        let upper_left = (-1.20, 0.35);
        let lower_right = (-1.0, 0.20);
        let mut pixels = vec![0; bounds.0 * bounds.1];

        b.iter(|| render(&mut pixels, bounds, upper_left, lower_right));
    }

    // TODO fix this test
    #[test] #[ignore]
    fn compare_gold_masters() {
        let tmp_dir = TempDir::new("output_tests").expect("create temp dir failed");
        println!("Generating test files in {:?}", tmp_dir);
        let filename = tmp_dir.path().join("mandel_4000x3000.png");
        let bounds = (4000, 3000);
        let upper_left = (-1.20, 0.35);
        let lower_right = (-1.0, 0.20);
        let mut pixels = vec![0; bounds.0 * bounds.1];

        render(&mut pixels, bounds, upper_left, lower_right);
        write_bitmap(&filename, &pixels, bounds).expect("error writing PNG file");
        println!("Written bitmap to '{:?}'", filename);

        // Compare output to the "golden master" file generated in the first version
        assert!(!dir_diff::is_different(&tmp_dir.path(), "gold_masters").unwrap());
    }
}

/// Write the buffer 'pixels', whose dimensions are given by 'bounds', to the file named 'filename'
fn write_bitmap(filename: &PathBuf, pixels: &[u8], bounds: (usize, usize)) -> Result<()> {
    let output = File::create(filename)?;

    let encoder = PNGEncoder::new(output);
    encoder.encode(&pixels, bounds.0 as u32, bounds.1 as u32,
        ColorType::Gray(8))?;

    Ok(())
}