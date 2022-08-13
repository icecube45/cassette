use std::collections::HashMap;

use ndarray::Array1;

use super::pixel::Pixel;
use super::frame::Frame;


// The patching layer is what does the translation from pixels in a frame
// to pixels being sent to the appropriate device in the appropriate order.

// Not all pixels in a frame need to be patched.
// For example, circular matrices - they'll use a rectangular frame for 
// animations, but they'll only need to patch the pixels that are in the
// circular region of the matrix.

// An output (and subsequently, a frame) can have multiple devices associated
// with it. For instance, an output (and the frame for that output) could be one large
// matrix, and the other outputs could be smaller matrices. This is what I usually
// refer to as the "duel monitor" setup - because we're nerds and understand that.

// The Patching layer handles output device pixel order as well - it generates a 
// one dimensional array of pixels that can be sent to the output device, from a two
// dimensional input array of pixels.

// Or take the example of a bike (like I will _hopefully_ build - and what cassette is
// being built for). The bike has a triangular frame, which has a matrix fit within it.
// The bike also has two wheels, which have circular pseudo-matrices (not important, 
// just imagine circular matrices).
// For this scenario, it's likely that one Ouptut will be the bike as a whole.
// In this case, a large rectangle would be used for the output frame, and the
// patching layer would be used to not only map to oddly sized matrices (like the frame
// triangle and the wheel circles), and to discrete devices (the wheels and the frame matrix)
// but also to map the physical location of the wheels and frame to each other - so that
// if you were to display an image on the output, it would act as though you were
// projecting an image onto the bike as a whole (an extension of the "duel monitor" setup).

// TODO: Actually make devices. I just need this here so I can template stuff out for now.
#[derive(Eq, Hash, PartialEq)]
struct Device{

}


struct PixelToPixel{
    // the x and y coordinates of the pixel in the frame
    x: usize,
    y: usize,
    // index of the pixel in the output array
    output_index: usize,
}
impl PixelToPixel{
    fn new(x: usize, y: usize, output_index: usize) -> PixelToPixel{
        PixelToPixel{
            x,
            y,
            output_index,
        }
    }
    
    fn patch(&self, frame: &mut Frame, output: &mut Array1<Pixel>){
        output[self.output_index] = frame.pixels[[self.y, self.x]];
    }
}

pub struct Patcher {
    // Devices that are patched to this output
    devices: Vec<Device>,
    // this map is used to map input frame pixels to their position in the output array
    pixel_map: HashMap<Device, Vec<PixelToPixel>>,
    // this map is used to store output pixels, which will be sent to the output device
    pixel_output_map: HashMap<Device, Array1<Pixel>>,
}

impl Patcher {
    pub fn new() -> Patcher {
        Patcher {
            devices: Vec::new(),
            pixel_map: HashMap::new(),
            pixel_output_map: HashMap::new(),
        }
    }

    pub fn patch(&mut self, frame: &mut Frame){
        for device in self.devices.iter(){
            let mut output_pixels = self.pixel_output_map.get_mut(device).unwrap();
            for pixel_to_pixel in self.pixel_map.get(device).unwrap().iter(){
                pixel_to_pixel.patch(frame, output_pixels);
            }
            // TODO: pass generated output to output device
        }
    }
}
