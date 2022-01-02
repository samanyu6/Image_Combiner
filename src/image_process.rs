use std::{io::BufReader, fs::File};
use image::{DynamicImage, ImageFormat, io::Reader, GenericImageView, imageops::FilterType::Triangle};

pub struct FloatingImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
    name: String,
}

impl FloatingImage {
    pub fn new(width: u32, height: u32, name: String) -> Self{
        let buffer_capacity = 3_655_744;
        let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity);

        FloatingImage { width: width, height: height, data: buffer, name: name }
    }
}

pub fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
    let image_reader: Reader<BufReader<File>> = Reader::open(path).unwrap();
    let image_format: ImageFormat = image_reader.format().unwrap();
    let image: DynamicImage = image_reader.decode().unwrap();

    (image, image_format)
}

pub fn get_smallest_dimensions(dim1: (u32, u32), dim2: (u32, u32)) -> (u32, u32) {
    let pix1 = dim1.0 * dim2.1;
    let pix2 = dim2.0 * dim2.1;

    return if pix1 < pix2 {dim1} else {dim2};
}

pub fn standardise_size(image1: DynamicImage, image2: DynamicImage) -> (DynamicImage, DynamicImage) {
    let (width, height) = get_smallest_dimensions(image1.dimensions(), image2.dimensions());
    println!("width: {}, height: {}\n", width, height);

    if(image2.dimensions() == (width, height)){
        (image1.resize_exact(width, height, Triangle), image2)
    } else {
        (image1, image2.resize_exact(width, height, Triangle))
    }
}

pub fn combine_images(image1: DynamicImage, image2: DynamicImage) -> Vec<u8> {
    let vec1 = image1.to_rgba8().to_vec();
    let vec2 = image2.to_rgba8().to_vec();

    alternate_pixels(vec1, vec2)
}

fn alternate_pixels(vec1: Vec<u8>, vec2: Vec<u8>) -> Vec<u8> {
    let mut combined_data = vec![0u8; vec1.len()];

    let mut i = 0;
    while i < vec1.len() {
        if i%8 == 0{
            combined_data.splice(i..=i + 3, set_rgba(&vec1, i, i+3));
        } else {
            combined_data.splice(i..=i + 3, set_rgba(&vec2, i, i + 3));
        }

        i+=4;
    }

    combined_data
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
    let mut rgba = Vec::new();
    for i in start..=end {
        let val = match vec.get(i) {
            Some(d) => *d,
            None => panic!("Index out of bounds")
        };

        rgba.push(val);
    }

    rgba
}
