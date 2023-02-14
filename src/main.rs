use std::time::Duration;
use rodio::{OutputStream, Sink};
use rodio::source::{SineWave, Source};

use std::fs::File;
use std::io::{BufReader};
use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};

// TODO match up notes with frequencies and print as the scale is playing
// TODO decide whehter to use CSV or stick with middle C/A as the basis for plahing notes
// Create CLI to add input of root note and scale type to command line

#[derive(Debug, Serialize, Deserialize)]
struct Note {
    note: String,
    frequency: f32,
    wavelength: f32,
}

fn parse_csv(path: &str) -> Vec<Note> {
    let mut result: Vec<Note> = Vec::new();
    let file = File::open(path).expect("Unable to open file");
    let buf_reader = BufReader::new(file);
    let mut csv_reader = ReaderBuilder::new().has_headers(true).from_reader(buf_reader);

    for row in csv_reader.deserialize() {
        let data: Note = row.expect("Unable to parse");
        result.push(data);
        //println!("{:?}", data);
    }

    result
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
    const MULT: f32 = 1.05946309436; //The 12th root of 2.

    let mut scale = if scale_type == "major" {
        vec![1.0, MULT.powf(2.0), MULT.powf(4.0), MULT.powf(5.0), MULT.powf(7.0),
        MULT.powf(9.0), MULT.powf(11.0), MULT.powf(12.0)]
    } 
    else if scale_type == "minor" {
        vec![1.0, MULT.powf(2.0), MULT.powf(3.0), MULT.powf(5.0), MULT.powf(7.0),
        MULT.powf(8.0), MULT.powf(10.0), MULT.powf(12.0)]
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
    let note_input = "C5";
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let notes = parse_csv("src/notes.csv");
    let root_note: Vec<Note> = notes.into_iter().filter(|n| n.note == note_input).collect();
    //println!("{:?}", build_scale(vec));
    let maj_scale = build_scale(root_note[0].frequency, "major");
    play_scale(sink, maj_scale);
}
