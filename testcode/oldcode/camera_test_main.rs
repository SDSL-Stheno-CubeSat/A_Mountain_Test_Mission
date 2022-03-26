use image::{ImageBuffer, Rgb};
use nokhwa::{query_devices, CallbackCamera, CameraIndex, CaptureAPIBackend};

fn main() {
    let cameras = query_devices(CaptureAPIBackend::Auto).unwrap();
    cameras.iter().for_each(|cam| println!("{:?}", cam));

    let mut threaded = CallbackCamera::new(&CameraIndex::Index(0), None).unwrap();
    

    threaded.open_stream().unwrap();
    // NOT THREADED -> REGULAR
    #[allow(clippy::empty_loop)] // keep it running
    loop {
        let frame = threaded.poll_frame().unwrap();
        println!(
            "{}x{} {} naripoggers",
            frame.width(),
            frame.height(),
            frame.len()
        );
    }
}

fn callback(image: ImageBuffer<Rgb<u8>, Vec<u8>>) {
    println!("{}x{} {}", image.width(), image.height(), image.len());

    let _res = image.save("image1.jpg");

    //let buffer: &[u8] = &image.into_raw(); // Generate the image data

    // Save the buffer as "image.png"
    //image::save_buffer("image.png", image, 800, 600, image::ColorType::Rgb8).unwrap();
}
