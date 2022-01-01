use std::{io::BufReader, fs::File};
use image::{DynamicImage, ImageFormat, io::Reader, GenericImageView, imageops::FilterType::Triangle};

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
