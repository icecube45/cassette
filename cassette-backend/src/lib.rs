// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }

    use std::{ops::Mul, cmp::{max, min}};

    use ndarray::{Ix2, Array, Axis};


    #[derive(Clone, Copy)]
    pub struct Pixel{
        pub r: u8,
        pub g: u8,
        pub b: u8,
    }

    impl Pixel{
        fn from_rgb(r: u8, g: u8, b: u8) -> Pixel {
            Pixel{
                r,
                g,
                b,
            }
        }

        fn black() -> Pixel {
            Pixel{
                r: 0,
                g: 0,
                b: 0,
            }
        }
        fn red() -> Pixel {
            Pixel{
                r: 255,
                g: 0,
                b: 0,
            }
        }
        fn green() -> Pixel {
            Pixel{
                r: 0,
                g: 255,
                b: 0,
            }
        }
        fn blue() -> Pixel {
            Pixel{
                r: 0,
                g: 0,
                b: 255,
            }
        }
        fn get_intensity(&self) -> f32 {
            ((self.r + self.g + self.b) / 3) as f32 / 255f32
        }
        fn is_transparent(&self) -> bool {
            self.r == 0 && self.g == 0 && self.b == 0
        }
    }

    impl Mul for Pixel {
        type Output = Self;
        fn mul(self, other: Self) -> Self {
            Pixel{
                r: self.r * other.r,
                g: self.g * other.g,
                b: self.b * other.b,
            }
        }
    }

    impl Mul<f32> for Pixel {
        type Output = Self;
        fn mul(self, other: f32) -> Self {
            Pixel{
                r: (self.r as f32 * other) as u8,
                g: (self.g as f32 * other) as u8,
                b: (self.b as f32 * other) as u8,
            }
        }
    }



    pub struct Frame{
        pub pixels: Array::<Pixel, Ix2>,
        width: u32,
        height: u32,
        timestamp: u128,
        

    }

    impl Frame{
        pub fn new(width: u32, height: u32) -> Frame {
            Frame{
                pixels: Array::from_elem((height as usize, width as usize), Pixel::black()),
                width,
                height,
                timestamp: 0,
            }
        }
        pub fn update_timestamp(&mut self) {
            self.timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
        }
        pub fn timestamp(&self) -> u128 {
            self.timestamp
        }
        pub fn width(&self) -> usize {
            self.width as usize
        }
        pub fn height(&self) -> usize {
            self.height as usize
        }
    }


    enum MixMode {
        Progressive,
        Linear,
        LeftShape,
        RightShape,
        LeftIntensity,
        RightIntensity,
        LeftOverlay,
        RightOverlay,
        LeftOverlayBorder,
        RightOverlayBorder,
    }

    struct Mixer{
        mix_mode: MixMode,
        mix_weight: f32,
    }

    // use Vec<Vec<Pixel>> as Frame;
    impl Mixer {
        fn progressive(&mut self, first_channel: Frame, second_channel: Frame) -> Frame {
            let mut result = Frame::new(first_channel.width, first_channel.height);
            for i in 0..first_channel.height() {
                for j in 0..first_channel.width() {
                    let first_pixel = first_channel.pixels[[i,j]];
                    let second_pixel = second_channel.pixels[[i,j]];
                    let r = (first_pixel.r as f32 * (self.mix_weight as f32 / 100.0)) + (second_pixel.r as f32 * (1.0 - (self.mix_weight as f32 / 100.0)));
                    let g = (first_pixel.g as f32 * (self.mix_weight as f32 / 100.0)) + (second_pixel.g as f32 * (1.0 - (self.mix_weight as f32 / 100.0)));
                    let b = (first_pixel.b as f32 * (self.mix_weight as f32 / 100.0)) + (second_pixel.b as f32 * (1.0 - (self.mix_weight as f32 / 100.0)));
                    let r = r as u8;
                    let g = g as u8;
                    let b = b as u8;
                    let pixel = Pixel{r, g, b};
                    result.pixels[[i,j]] = pixel;
                }
            }
            result.update_timestamp();
            return result;
        }

        fn linear(&mut self, first_channel: Frame, second_channel: Frame) -> Frame {
            let mut result = Frame::new(first_channel.width, first_channel.height);
            for i in 0..first_channel.height as usize {
                for j in 0..first_channel.width as usize {
                    let first_pixel = first_channel.pixels[[i,j]];
                    let second_pixel = second_channel.pixels[[i,j]];
                    let r = (first_pixel.r as f32 * (self.mix_weight as f32 / 100.0)) + (second_pixel.r as f32 * (1.0 - (self.mix_weight as f32 / 100.0)));
                    let g = (first_pixel.g as f32 * (self.mix_weight as f32 / 100.0)) + (second_pixel.g as f32 * (1.0 - (self.mix_weight as f32 / 100.0)));
                    let b = (first_pixel.b as f32 * (self.mix_weight as f32 / 100.0)) + (second_pixel.b as f32 * (1.0 - (self.mix_weight as f32 / 100.0)));
                    let r = r as u8;
                    let g = g as u8;
                    let b = b as u8;
                    let pixel = Pixel{r, g, b};
                    result.pixels[[i,j]] = pixel;
                }
            }
            result.update_timestamp();
            return result;
        }

        fn shape(&mut self, first_channel: Frame, second_channel: Frame) -> Frame {
            let mut result = Frame::new(first_channel.width, first_channel.height);
            for i in 0..first_channel.height() {
                for j in 0..first_channel.width() {
                    if(!first_channel.pixels[[i,j]].is_transparent()){
                        let pixel = second_channel.pixels[[i,j]];
                        result.pixels[[i,j]] = pixel;
                    } else {
                        result.pixels[[i,j]] = Pixel::black();
                    }
                }
            }
            result.update_timestamp();
            return result;
        }

        fn intensity(&mut self, first_channel: Frame, second_channel: Frame) -> Frame {
            let mut result = Frame::new(first_channel.width, first_channel.height);
            for i in 0..first_channel.height() {
                for j in 0..first_channel.width() {
                    result.pixels[[i,j]] = second_channel.pixels[[i,j]]*first_channel.pixels[[i,j]].get_intensity();
                }
            }
            result.update_timestamp();
            return result;
        }

        fn overlay(&mut self, first_channel: Frame, second_channel: Frame) -> Frame {
            let mut result = Frame::new(first_channel.width, first_channel.height);
            for i in 0..first_channel.height() {
                for j in 0..first_channel.width() {
                    if(!first_channel.pixels[[i,j]].is_transparent()){
                        let pixel = first_channel.pixels[[i,j]];
                        result.pixels[[i,j]] = pixel;
                    } else {
                        result.pixels[[i,j]] = second_channel.pixels[[i,j]];
                    }
                }
            }
            result.update_timestamp();
            return result;
        }

        fn border(&mut self, first_channel: Frame, second_channel: Frame) -> Frame {
            let mut result = Frame::new(first_channel.width, first_channel.height);
            for i in 0..first_channel.height() {
                for j in 0..first_channel.width() {
                    if(!first_channel.pixels[[i,j]].is_transparent()){
                        let pixel = first_channel.pixels[[i,j]];
                        result.pixels[[i,j]] = pixel;
                    } else {
                        // check surrounding pixels to see if they are not transparent
                        let mut border = false;
                        if(i > 0){
                            if(!first_channel.pixels[[i-1,j]].is_transparent()){
                                border = true;
                            } else if(i<first_channel.height()-2){
                                if(!first_channel.pixels[[i+1,j]].is_transparent()){
                                    border = true;
                                }
                            }
                        }
                        if(j > 0){
                            if(!first_channel.pixels[[i,j-1]].is_transparent()){
                                border = true;
                            } else if(j<first_channel.width()-2){
                                if(!first_channel.pixels[[i,j+1]].is_transparent()){
                                    border = true;
                                }
                            }
                        }
                        if(border){
                            result.pixels[[i,j]] = Pixel::black();
                        } else {
                            result.pixels[[i,j]] = second_channel.pixels[[i,j]];
                        }
                    }
                }
            }
            result.update_timestamp();
            return result;
        }


        // given two frames of pixels, 
        fn mix(&mut self, first_channel: Frame, second_channel: Frame) -> Frame{
            match self.mix_mode {
                // Will do a simple crossfade of the two effects, where every effect reaches 100% in the middle of the mix fader.
                // TODO make this actually correct with 100% at center, I think it's just linear now.
                MixMode::Progressive => {
                    return Mixer::progressive(self, first_channel, second_channel);
                }
                // A classic cross fader where every effect will reach 100% at the opposite end of the fader.
                MixMode::Linear => {
                    return Mixer::linear(self, first_channel, second_channel);
                }
                // the shape of one animation is colored with the second animation
                MixMode::LeftShape => {
                    return Mixer::shape(self, first_channel, second_channel);
                }
                // the shape of one animation is colored with the second animation
                MixMode::RightShape => {
                    return Mixer::shape(self, second_channel, first_channel);
                }
                //Nearly the same as Upper/Lower Shape, but instead of the shape the color intensity will be used.
                MixMode::LeftIntensity => {
                    return Mixer::intensity(self, first_channel, second_channel);
                }
                //Nearly the same as Upper/Lower Shape, but instead of the shape the color intensity will be used.
                MixMode::RightIntensity => {
                    return Mixer::intensity(self, second_channel, first_channel);
                }
                // This will overlay one effect over the other. Every black pixel from the overlaying effect will be treated as transparent.
                MixMode::LeftOverlay => {
                    return Mixer::overlay(self, first_channel, second_channel);
                }
                // This will overlay one effect over the other. Every black pixel from the overlaying effect will be treated as transparent.
                MixMode::RightOverlay => {
                    return Mixer::overlay(self, second_channel, first_channel);
                }
                // Same as Upper/Lower Overlay, but a small border will be drawn on the overlaying effect.
                MixMode::LeftOverlayBorder => {
                    return Mixer::border(self, first_channel, second_channel);
                }
                // Same as Upper/Lower Overlay, but a small border will be drawn on the overlaying effect.
                MixMode::RightOverlayBorder => {
                    return Mixer::border(self, second_channel, first_channel);
                }                    
            }
        }
        fn set_mix_mode(&mut self, mode: MixMode){
            // set the mix mode
            self.mix_mode = mode;
        }
        fn set_mix_weight(&mut self, weight: u8){
            // set the mix weight, bound it between 0 and 100
            self.mix_weight = max(0, min(weight, 100)) as f32;
        }
    }


    pub struct RainbowWheel {
        // the current position of the rainbow wheel
        step: u8,
    }
    pub trait Animation {
        fn generate_frame(&mut self, width: u32, height: u32) -> Frame;
    }


    impl RainbowWheel {
        // fn strip_to_matrix(frame: Frame) -> Frame {
        //     let mut copy = frame.pixels.clone();
        //     let strip = copy.row_mut(0);
        //     for mut row in frame.pixels.axis_iter_mut(Axis(0)) {
        //         row = strip;
        //     }
        //     return frame;
        // }
        
        fn rainbow(&mut self, mut frame: Frame, matrix: bool) -> Frame {
            let mut num_pixels_override = frame.height()*frame.width();
            if !matrix {
                num_pixels_override = frame.width();
            }
            for i in 0..num_pixels_override {
                let pixel_index = (i*256/num_pixels_override) + self.step as usize;
                let (r, g, b) = RainbowWheel::wheel(pixel_index as u8);
                let pixel = Pixel::from_rgb(r, g, b);
                if !matrix {
                    frame.pixels[[0,i]] = pixel;
                } else {
                    let width = frame.width();
                    frame.pixels[[i/width, i%width]] = pixel;
                }
            }
            if(self.step == 255){
                self.step = 0;
            } else {
                self.step += 1;
            }
            if !matrix{
                // frame = RainbowWheel::strip_to_matrix(frame);
            }
            return frame;
        }

        fn wheel(mut n: u8) -> (u8, u8, u8) {
            let mut r: u8 = 0;
            let mut g: u8 = 0;
            let mut b: u8 = 0;
            if n < 85 {
                r = n * 3;
                g = 255-n*3;
                b = 0;
            }
            else if n < 170 {
                n = n-85;
                r = 255 - n*3;
                g = 0;
                b = n*3;
            }
            else {
                n = n-170;
                r = 0;
                g = n*3;
                b = 255 - n*3;
            }
            return (r, g, b);
        }        
        pub fn new() -> RainbowWheel {
            return RainbowWheel {
                step: 0,
            };
        }
    }

    impl Animation for RainbowWheel {
        

        fn generate_frame(&mut self, width: u32, height: u32) -> Frame {
            let mut frame = Frame::new(width, height);
            frame = RainbowWheel::rainbow(self, frame, true);
            frame.update_timestamp();
            return frame;
        }
    }
            



