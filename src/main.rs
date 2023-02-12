use std::time::Duration;
use rodio::{OutputStream, Sink};
use rodio::source::{SineWave, Source};

use std::fs::File;
use std::io::{BufReader};
use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};

// TODO match up notes with frequencies and print as the scale is playing
// TODO update Data struct to pull in frequency with note name
// TODO decide whehter to use CSV or stick with middle C/A as the basis for plahing notes

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    Note: String,
    Frequency: f64,
    Wavelength: f64,
}

fn parse_csv(path: &str) {
    let mut result: Vec<Data> = Vec::new();
    let file = File::open(path).expect("Unable to open file");
    let buf_reader = BufReader::new(file);
    let mut csv_reader = ReaderBuilder::new().has_headers(true).from_reader(buf_reader);

    for result in csv_reader.deserialize() {
        let data: Data = result.expect("Unable to parse");
        //result.push(data);
        println!("{:?}", data);
    }

    // result
}

fn play_scale(sink: Sink, scale: Vec<f32>) {
    for step in scale {
        let note = SineWave::new(step).take_duration(Duration::from_secs_f32(0.5)).amplify(0.20);
        sink.append(note);
        sink.sleep_until_end();
    }
}

fn build_scale(root: f32, scale_type: &str) -> Vec<f32> {
    //major scale: whole whole half whole whole half whole
    //minor scale: whole half whole whole half whole whole
    const mult: f32 = 1.05946309436; //The 12th root of 12.

    let mut scale = if scale_type == "major" {
        vec![1.0, mult.powf(2.0), mult.powf(4.0), mult.powf(5.0), mult.powf(7.0),
        mult.powf(9.0), mult.powf(11.0), mult.powf(12.0)]
    } 
    else if scale_type == "minor" {
        vec![1.0, mult.powf(2.0), mult.powf(3.0), mult.powf(5.0), mult.powf(7.0),
        mult.powf(8.0), mult.powf(10.0), mult.powf(12.0)]
    }
    else {
        println!("Please select major or minor as your scale type.");
        vec![]
    };

    for i in scale.iter_mut() {
        *i *= root;
     }
    scale
}


fn main() {
    let root: f32 = 440.0; //A4
    let mult: f32 = 1.0595;
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let c_maj: [f32; 8] = [523.25, 587.33, 659.25,698.46,783.99,880.0,987.77,1046.50];

    
    //println!("{:?}", build_scale(vec));
    let maj_scale = build_scale(root, "major");
    let min_scale = build_scale(root, "minor");
    let bad_scale = build_scale(root, "bad");
    //println!("{:?}", scale_test);
    play_scale(sink, maj_scale);
    //parse_csv("src/notes.csv");
}
