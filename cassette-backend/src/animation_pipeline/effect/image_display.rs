use std::io::Read;
use std::{sync::Arc};

use image::AnimationDecoder;
use image::DynamicImage;

use parking_lot::Mutex;

use tokio::sync::mpsc::{self, Sender, Receiver};

use image::GenericImageView;
use image::Pixel;
use image::codecs::gif::GifDecoder;
use crate::{dsp::{DSP}};

use super::{Frame, Animate};
pub struct ImageDisplay {
    tempo_event: bool,
    new_image: bool,
    sync_to_tempo: bool,
    dsp: Arc<Mutex<DSP>>,
    tempo_channel_rx: mpsc::Receiver<bool>,
    last_frame: Frame,
    image: image::DynamicImage,
    frames: Vec<DynamicImage>,
    frame_index: usize,
    is_gif: bool,
} 

impl ImageDisplay {
    pub fn new(dsp: Arc<Mutex<DSP>>) -> Self {
        let (tx, rx): (Sender<bool>, Receiver<bool>) = mpsc::channel(10);
        let is_gif = false;
        let mut image = DynamicImage::new_rgb8(0, 0);
        // create empty frames
        let mut frames = Vec::<DynamicImage>::new();
        if is_gif {
            let file_in = std::fs::File::open("doge.gif").unwrap();
            let decoder = GifDecoder::new(file_in).unwrap();
            let frames_native = decoder.into_frames();
            let gif_frames = frames_native.collect_frames().expect("error decoding gif");
            for gif_frame in gif_frames{
                let rgba = gif_frame.into_buffer();
                frames.push(DynamicImage::ImageRgba8(rgba));

            }
            // convert to image
            // convert rgba to image
            
        }
        else{
            // for every png in folder, create frame
            let path = std::path::Path::new("src/animation_pipeline/effect/images");
            let display = path.display();
            let mut files = std::fs::read_dir(path).expect(&format!("could not read {}", display));
            let mut i = 0;
            while let Some(file) = files.next() {
                let file = file.unwrap();
                let path = file.path();
                let display = path.display();
                let mut file_in = std::fs::File::open(&path).expect(&format!("could not read {}", display));
                let mut buf = Vec::<u8>::new();
                file_in.read_to_end(&mut buf).expect(&format!("could not read {}", display));
                let image = image::load_from_memory(&buf).expect(&format!("could not read {}", display));
                let frame = image;
                frames.push(frame);
                i += 1;
            }
        }
        let mut imgDisplay = ImageDisplay{
            tempo_event: false,
            new_image: true,
            sync_to_tempo: true,
            dsp,
            tempo_channel_rx: rx,
            last_frame: Frame::new(0, 0),
            image,
            frames,
            is_gif,
            frame_index: 0,
            };
        if imgDisplay.sync_to_tempo {
            let mut dsp = imgDisplay.dsp.lock();
            // create arc mutex of squares
            // create tempo callback
            dsp.add_tempo_channel(tx.clone());
        }
    imgDisplay
    }

}


impl Animate for ImageDisplay {
    fn animate(&mut self, frame: &mut Frame) {
        
        self.display_image(frame);
    }
}

impl ImageDisplay{
    pub fn display_image(&mut self, frame: &mut Frame){
        // receive all tempo_channel_rx events
        if self.sync_to_tempo {
            while self.tempo_channel_rx.try_recv().is_ok() {
                // receive
                self.tempo_event = true;
            }
        }
        if self.new_image || self.tempo_event {
            if self.tempo_event {
                self.frame_index = self.frame_index + 1;
                if self.frame_index >= self.frames.len() {
                    self.frame_index = 0;
                }
                self.image = self.frames[self.frame_index].clone();
        }


            // println!("new image");
            let img = &self.image;
            let img_width = img.width() as u32;
            let img_height = img.height() as u32;
            // let img_pixels = img.pixels();
            
            // calculate best way to resize image to fit within frame while maintaining aspect ratio
            let mut img_scale = 1.0;
            let mut img_width_scale = 1.0;
            let mut img_height_scale = 1.0;
            img_width_scale = (frame.width() as f32) / (img_width as f32);
            img_height_scale = (frame.height() as f32) / (img_height as f32);
            img_scale =f32::min(img_width_scale, img_height_scale);
           
            let img_width_scaled = (img_width as f32) * img_scale;
            let img_height_scaled = (img_height as f32) * img_scale;
            // resize image
            let mut img_resized = img.resize(img_width_scaled as u32, img_height_scaled as u32, image::imageops::FilterType::Nearest);
            // let img_resized_pixels = img_resized.pixels();
            let img_resized_width = img_resized.width() as u32;
            let img_resized_height = img_resized.height() as u32;
            // println!("{}x{} resized to {}x{}", img_width, img_height, img_resized_width, img_resized_height);

            // draw image to frame
            for x in 0..img_resized_width {
                for y in 0..img_resized_height {
                    let pixel = img_resized.get_pixel(x, y);
                    let pixel_rgb = pixel.to_rgba();
                    let pixel_r = pixel_rgb[0] as u8;
                    let pixel_g = pixel_rgb[1] as u8;
                    let pixel_b = pixel_rgb[2] as u8;
                    let pixel_a = pixel_rgb[3] as u8;
                    let pixel_color = crate::animation_pipeline::pixel::Pixel::from_rgba(pixel_r, pixel_g, pixel_b, pixel_a);
                    frame.set_pixel(x, y, pixel_color);
                }
            }






            self.new_image = false;
            self.tempo_event = false;
            self.last_frame = frame.clone();
        }
        else{
            frame.copy_from(&self.last_frame);
            return;
        }

    }


}
