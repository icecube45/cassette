use crate::animation_pipeline::components::Pixel;

use super::Frame;

use rand::Rng;


fn animate_expanding_squares(eo_count: u8, eo_size: u8, eo_growth: u8, eo_objects_pos_x: &mut Vec<u8>, eo_objects_pos_y: &mut Vec<u8>, eo_objects_expand: &mut Vec<u8>, eo_objects_fade: &mut Vec<u8>, eo_objects_col: &mut Vec<Pixel>, frame: &mut Frame) {
    for j in 0..eo_count as usize{
        let r = (eo_objects_col[j].r as f32 * (1f32 - eo_objects_fade[j] as f32 / eo_growth as f32)) as u8;
        let g = (eo_objects_col[j].g as f32 * (1f32 - eo_objects_fade[j] as f32 / eo_growth as f32)) as u8;
        let b = (eo_objects_col[j].b as f32 * (1f32 - eo_objects_fade[j] as f32 / eo_growth as f32)) as u8;
        let pixel = Pixel{ r, g, b};
        let diameter = eo_size + eo_objects_expand[j];
        let height = frame.pixels.shape()[1];
        let width  = frame.pixels.shape()[0];

        if (eo_objects_expand[j] < eo_growth) {
            let offset = diameter / 2;
            // yeah this makes sense to be impl on the struct itself
            // https://github.com/icecube45/cassette/blob/backend_start/cassette-backend/src/lib.rs#L101 like here
            frame.pixels.drawRect((eo_objects_pos_x[j] as i32-offset as i32), (eo_objects_pos_y[j] as i32 - offset as i32), diameter as i32, diameter as i32, pixel);
        }
        eo_objects_expand[j] += 2;
        eo_objects_fade[j] += 2;
        if(eo_objects_fade[j] >= eo_growth){
            eo_objects_fade[j] = eo_growth;
        }
        // get random int in range
        let mut rng = rand::thread_rng();
        if(eo_objects_expand[j] >= eo_growth && rng.gen_range(0..100 as u8) < 10){
            eo_objects_expand[j] = 0;
            eo_objects_fade[j] = 0;
            eo_objects_pos_x[j] = rng.gen_range(0..width as u8);
            eo_objects_pos_y[j] = rng.gen_range(0..height as u8);
        }
    }
}