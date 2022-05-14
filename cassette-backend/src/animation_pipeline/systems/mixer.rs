pub enum MixMode {
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



pub struct Mixer{
    pub mix_mode: MixMode,
    pub mix_weight: f32,
}

// use Vec<Vec<Pixel>> as Frame;
impl Mixer {
    fn progressive(&mut self, first_channel: Frame, second_channel: Frame) -> Frame {
        //crossfade that mixes each frame at 100% when the weight is at 50
        let mut result = Frame::new(first_channel.width, first_channel.height);
        for j in 0..first_channel.height() {
            for i in 0..first_channel.width() {
                let first_pixel = first_channel.pixels[[j, i]];
                let second_pixel = second_channel.pixels[[j, i]];
                let mut first_mix_weight = 0.0;
                let mut second_mix_weight = 0.0;
                if(self.mix_weight < 0.5) {
                    first_mix_weight =  1.0;
                    second_mix_weight = self.mix_weight/100.0 * 2.0;
                } else {
                    first_mix_weight = (1.0 - self.mix_weight / 100.0) * 2.0;
                    second_mix_weight = 1.0;
                }
                let mut r = (first_pixel.r as f32 * first_mix_weight + second_pixel.r as f32 * second_mix_weight);
                let mut g = (first_pixel.g as f32 * first_mix_weight + second_pixel.g as f32 * second_mix_weight);
                let mut b = (first_pixel.b as f32 * first_mix_weight + second_pixel.b as f32 * second_mix_weight);
                if(r > 255.0) {
                    r = 255.0;
                }
                if(g > 255.0) {
                    g = 255.0;
                }
                if(b > 255.0) {
                    b = 255.0;
                }
                let r = r as u8;
                let g = g as u8;
                let b = b as u8;
                let pixel = Pixel{r, g, b};
                result.pixels[[j,i]] = pixel;
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
    pub fn mix(&mut self, first_channel: Frame, second_channel: Frame) -> Frame{
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