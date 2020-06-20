
use log::trace;
use log::debug;
use log::info;

use crate::util::get_env_as_f32;

const INITIALIZING_FINISHED:u8 = 0;

pub struct Simple {
    indeces: [u32; 100],
    current_pixel_avg: u8,
    init_pixel_diff: f32,
}

impl Simple {
    pub fn new() -> Simple {
        trace!("new()");
        let pixel_diff = get_env_as_f32("INIT_PIXEL_DIFF");
        Simple {
            current_pixel_avg: 255,
            indeces: [0; 100],
            init_pixel_diff: pixel_diff,
        }
    }

    pub fn ingest(&mut self, img: &Vec<u8>) -> Result<(), String> {
        trace!("ingest()");
        if !self.initialized() {
            return self.initialize(img);
        }
        self.motion(img)
    }

    fn initialized(&mut self) -> bool {
        self.current_pixel_avg == INITIALIZING_FINISHED
    }

    fn initialize(&mut self, img: &Vec<u8>) -> Result<(), String> {
        trace!("initialize()");
        if self.indeces[0] == 0 {
            self.initialize_indeces(img);
        }
        let new_pixel_average = self.get_pixel_average(img);
        let pixel_difference = self.get_pixel_diff_as_percent(new_pixel_average);
        debug!("current_pixel_average={} new_pixel_average={} pixel_difference={} ALLOWANCE={}", 
            self.current_pixel_avg, new_pixel_average, pixel_difference, self.init_pixel_diff);
        if pixel_difference >= self.init_pixel_diff {
            info!("initialized");
            self.current_pixel_avg = INITIALIZING_FINISHED;
        } else {
            self.current_pixel_avg = new_pixel_average;
        }
        Ok(())
    }

    fn get_pixel_diff_as_percent(&mut self, new_pixel_avg:u8) -> f32 {
        trace!("get_pixel_diff_as_percent()");
        let max:u8;
        let min:u8;
        if new_pixel_avg >= self.current_pixel_avg {
            max = new_pixel_avg;
            min = self.current_pixel_avg;
        } else {
            max = self.current_pixel_avg;
            min = new_pixel_avg;
        }
        min as f32 / max as f32
    }

    // build the list of self.indeces
    fn initialize_indeces(&mut self, img: &Vec<u8>) {
        trace!("initialize_indeces()");
        self.indeces[0] = 1;
        let mut index:u32 = 0;
        let increment:u32 = (img.len() / self.indeces.len()) as u32;
        for i in 1..self.indeces.len() {
            self.indeces[i] = index;
            index += increment;
        }
    }

    // gets the average of the pixels at the indeces found
    // in self.indeces
    fn get_pixel_average(&mut self, img: &Vec<u8>) -> u8 {
        trace!("get_pixel_average()");
        let img_len:usize = img.len();
        let last_index:u32 = self.indeces[self.indeces.len() - 1];
        if last_index >= (img_len as u32) {
            self.initialize_indeces(img);
        }
        let mut total:u32 = 0;
        let mut avg:u8 = 0;
        for index in 0..self.indeces.len() {
            let pixel:u8 = img[index];
            let count:u32 = (index + 1) as u32; // usize -> u32
            total = total + (pixel as u32); // u8 -> u32
            avg = (total / count) as u8; // u32 -> u8
        }
        return avg
    }

    fn motion(&mut self, _img: &Vec<u8>) -> Result<(), String> {

        Ok(())
    }
}