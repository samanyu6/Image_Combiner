
mod args;
use args::Args;

mod image_process;
use image::GenericImageView;
use image_process::{find_image_from_path, standardise_size, FloatingImage, combine_images};

enum ImageDataErrors {
    DifferentImageFormats,
}

fn main() ->  Result<(), ImageDataErrors>{
    let args = Args::new();

    let (image1, image1_format) = find_image_from_path(args.image_1);
    let (image2, image2_format) = find_image_from_path(args.image_2);

    if image1_format!=image2_format {
        return Err(ImageDataErrors::DifferentImageFormats);
    }

    let (image1, image2) = standardise_size(image1, image2);
    let mut output = FloatingImage::new(image1.width(), image1.height(), args.output);
    let combined_images = combine_images(image1, image2);

    Ok(());
    // println!("{:?}", args);
}
