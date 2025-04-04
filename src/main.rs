use opencv::prelude::*;

fn main() {

    let mut cam = opencv::videoio::VideoCapture::new(0, opencv::videoio::CAP_ANY).unwrap();
    if !cam.is_opened().unwrap() {
        panic!("Camera not opened");
    }
    let opened = cam.is_opened().unwrap();
    if !opened {
        panic!("Camera not opened");
    }
    // Capture one frame and save it to a file
    let mut frame = opencv::core::Mat::default().unwrap();
    cam.read(&mut frame).unwrap();
    if frame.empty().unwrap() {
        panic!("Frame is empty");
    }
    let filename = "frame.jpg";
    opencv::imgcodecs::imwrite(filename, &frame, &opencv::core::Vector::new()).unwrap();
    println!("Frame saved to {}", filename);
}
