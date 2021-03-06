#![allow(unused_parens)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_upper_case_globals)]

extern crate libm;
extern crate num_traits;

use std::fs::File;
use std::io::Write;
use std::env;

use num_traits::{cast::FromPrimitive, float::Float};

pub trait FaustDsp {
    type Sample;

    fn new() -> Self where Self: Sized;
    fn metadata(&self, m: &mut dyn Meta);
    fn get_sample_rate(&self) -> i32;
    fn get_num_inputs(&self) -> i32;
    fn get_num_outputs(&self) -> i32;
    fn get_input_rate(&self, channel: i32) -> i32;
    fn get_output_rate(&self, channel: i32) -> i32;
    fn class_init(sample_rate: i32) where Self: Sized;
    fn instance_reset_user_interface(&mut self);
    fn instance_clear(&mut self);
    fn instance_constants(&mut self, sample_rate: i32);
    fn instance_init(&mut self, sample_rate: i32);
    fn init(&mut self, sample_rate: i32);
    fn build_user_interface(&mut self, ui_interface: &mut dyn UI<Self::Sample>);
    fn compute(&mut self, count: i32, inputs: &[&[Self::Sample]], outputs: &mut[&mut[Self::Sample]]);
}

pub trait Meta {
    // -- metadata declarations
    fn declare(&mut self, key: &str, value: &str);
}

pub trait UI<T> {
    // -- widget's layouts
    fn open_tab_box(&mut self, label: &str);
    fn open_horizontal_box(&mut self, label: &str);
    fn open_vertical_box(&mut self, label: &str);
    fn close_box(&mut self);

    // -- active widgets
    fn add_button(&mut self, label: &str, zone: &mut T);
    fn add_check_button(&mut self, label: &str, zone: &mut T);
    fn add_vertical_slider(&mut self, label: &str, zone: &mut T, init: T, min: T, max: T, step: T);
    fn add_horizontal_slider(&mut self, label: &str, zone: &mut T , init: T, min: T, max: T, step: T);
    fn add_num_entry(&mut self, label: &str, zone: &mut T, init: T, min: T, max: T, step: T);

    // -- passive widgets
    fn add_horizontal_bargraph(&mut self, label: &str, zone: &mut T, min: T, max: T);
    fn add_vertical_bargraph(&mut self, label: &str, zone: &mut T, min: T, max: T);

    // -- metadata declarations
    fn declare(&mut self, zone: &mut T, key: &str, value: &str);
}

pub struct ButtonUI<T>
{
    fState: T
}

impl<T: Float + FromPrimitive> UI<T> for ButtonUI<T>
{
    // -- widget's layouts
    fn open_tab_box(&mut self, label: &str) {}
    fn open_horizontal_box(&mut self, label: &str) {}
    fn open_vertical_box(&mut self, label: &str) {}
    fn close_box(&mut self) {}

    // -- active widgets
    fn add_button(&mut self, label: &str, zone: &mut T)
    {
        //println!("addButton: {}", label);
        *zone = self.fState;
    }
    fn add_check_button(&mut self, label: &str, zone: &mut T) {}
    fn add_vertical_slider(&mut self, label: &str, zone: &mut T, init: T, min: T, max: T, step: T) {}
    fn add_horizontal_slider(&mut self, label: &str, zone: &mut T , init: T, min: T, max: T, step: T) {}
    fn add_num_entry(&mut self, label: &str, zone: &mut T, init: T, min: T, max: T, step: T) {}

    // -- passive widgets
    fn add_horizontal_bargraph(&mut self, label: &str, zone: &mut T, min: T, max: T) {}
    fn add_vertical_bargraph(&mut self, label: &str, zone: &mut T, min: T, max: T) {}

    // -- metadata declarations
    fn declare(&mut self, zone: &mut T, key: &str, value: &str) {}
}

// Generated intrinsics:
<<includeIntrinsic>>

// Generated class:
<<includeclass>>

const SAMPLE_RATE: i32 = 44100;

type Dsp64 = dyn FaustDsp<Sample=f64>;

fn print_header(mut dsp: Box<Dsp64>, num_total_samples: usize, output_file: &mut File) {
    dsp.init(SAMPLE_RATE);
    writeln!(output_file, "number_of_inputs  : {}", dsp.get_num_inputs()).unwrap();
    writeln!(output_file, "number_of_outputs : {}", dsp.get_num_outputs()).unwrap();
    writeln!(output_file, "number_of_frames  : {}", num_total_samples).unwrap();
}

fn run_dsp(mut dsp: Box<Dsp64>, num_samples: usize, line_num_offset: usize, output_file: &mut File) {
    type RealType = <Dsp64 as FaustDsp>::Sample;

    // Generation constants
    let buffer_size = 64usize;

    // Init dsp
    dsp.init(SAMPLE_RATE);

    let num_inputs = dsp.get_num_inputs() as usize;
    let num_outputs = dsp.get_num_outputs() as usize;

    // Prepare buffers
    let mut in_buffer = vec![vec![0 as RealType; buffer_size]; num_inputs];
    let mut out_buffer = vec![vec![0 as RealType; buffer_size]; num_outputs];

    // Compute
    let mut cycle = 0;
    let mut num_samples_written = 0;
    while num_samples_written < num_samples {

        let buffer_size = buffer_size.min(num_samples - num_samples_written);

        // handle inputs
        for c in 0..num_inputs {
            for j in 0..buffer_size {
                let first_frame = num_samples_written == 0 && j == 0;
                in_buffer[c][j] = if first_frame { 1.0 } else { 0.0 };
            }
        }

        // Set button state
        if cycle == 0 {
            let mut button_on = ButtonUI::<f64>{ fState: 1.0 };
            dsp.build_user_interface(&mut button_on);
        } else {
            let mut button_off = ButtonUI::<f64>{ fState: 0.0 };
            dsp.build_user_interface(&mut button_off);
        }

        dsp.compute(
            buffer_size as i32,
            in_buffer.iter().map(|buffer| buffer.as_slice()).collect::<Vec<&[RealType]>>().as_slice(),
            out_buffer.iter_mut().map(|buffer| buffer.as_mut_slice()).collect::<Vec<&mut [RealType]>>().as_mut_slice(),
        );

        // handle outputs
        for j in 0..buffer_size {
            write!(output_file, "{:6} :", num_samples_written + line_num_offset).unwrap();
            for c in 0..num_outputs {
                write!(output_file, " {:8.6}", out_buffer[c][j]).unwrap();
            }
            writeln!(output_file).unwrap();
            num_samples_written += 1;
        }

        cycle = cycle + 1;
    }
}

fn new_dsp() -> Box<Dsp64> {
    Box::new(mydsp::new())
}

fn main() {
    let num_total_samples = 60000;

    let block_size = num_total_samples / 4;

    // Open output file
    let output_file_name = env::args().nth(1).expect("ERROR: Output file name expected.");
    let mut output_file = File::create(output_file_name).expect("Cannot create output file");

    print_header(new_dsp(), num_total_samples, &mut output_file);

    // Only test mono DSP for now
    run_dsp(new_dsp(), block_size, 0, &mut output_file);

    //run_dsp(new_dsp(), block_size, 1 * block_size, &mut output_file);
    //run_dsp(new_dsp(), block_size, 2 * block_size, &mut output_file);
    //run_dsp(new_dsp(), block_size, 3 * block_size, &mut output_file);
}
