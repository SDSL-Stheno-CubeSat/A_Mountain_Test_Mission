use image::*;
use nokhwa::{query_devices, CameraFormat, FrameFormat, CaptureAPIBackend, Camera, CameraIndex};

fn main() {
    let cameras = query_devices(CaptureAPIBackend::Auto).unwrap();
    cameras.iter().for_each(|cam| println!("{:?}", cam));

    let mut camera = Camera::new(
        &CameraIndex::Index(2), // index
        Some(CameraFormat::new_from(640, 480, FrameFormat::MJPEG, 30)), // format
    ).unwrap();

    // open stream
    camera.open_stream().unwrap();

    // 640 x 480
    for x in 0..3 {
        let frame : ImageBuffer<Rgb<u8>, Vec<u8>> = camera.frame().unwrap();
        println!(
            "{}x{} {}",
            frame.width(),
            frame.height(),
            frame.len()
        );
        
        //frame.save(format!("/home/kubos/images/testimage{}.png", x)).unwrap();
    }
}
