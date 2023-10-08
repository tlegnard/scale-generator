use std::time::Duration;
use rodio::{OutputStream, Sink};
use rodio::source::{SineWave, Source};

use std::fs::File;
use std::io::{BufReader};
use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};

use clap::{Arg, App};

// TODO decide whehter to use CSV or stick with middle C/A as the basis for plahing notes

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    }

    result
}

fn match_root(notes: &Vec<Note>, root_note: &str) -> Option<usize> {
    for (i, note) in notes.iter().enumerate() {
        if note.note == root_note {
            return Some(i);
        }
    }
    None
}

fn build_scale(root: &Note, scale_type: &str) -> Vec<f32> {
    const MULT: f32 = 1.05946309436; //The 12th root of 2.

    let mut scale = if scale_type == "major" {
        //major scale: whole whole half whole whole half whole
        vec![1.0, MULT.powf(2.0), MULT.powf(4.0), MULT.powf(5.0), MULT.powf(7.0),
        MULT.powf(9.0), MULT.powf(11.0), MULT.powf(12.0)]
    } 
    else if scale_type == "minor" {
        //minor scale: whole half whole whole half whole whole
        vec![1.0, MULT.powf(2.0), MULT.powf(3.0), MULT.powf(5.0), MULT.powf(7.0),
        MULT.powf(8.0), MULT.powf(10.0), MULT.powf(12.0)]
    }
    else {
        println!("Please select major or minor as your scale type.");
        vec![]
    };

    for i in scale.iter_mut() {
        *i *= root.frequency;
     }
    scale
}

fn play_scale(sink: Sink, scale: Vec<f32>) {
    for step in scale {
        let note = SineWave::new(step).take_duration(Duration::from_secs_f32(0.5)).amplify(0.20);
        sink.append(note);
        sink.sleep_until_end();
    }
}


fn main() {
    let args: Vec<String> = std::env::args().collect();
    let matches = App::new("scale-generator")
        .arg(Arg::with_name("note")
            .short('n')
            .long("note")
            .value_name("NOTE")
            .takes_value(true)
            .help("The root note of the scale"))
        .arg(Arg::with_name("scale-type")
            .short('s')
            .long("scale-type")
            .takes_value(true)
            .possible_values(&["major", "minor"])
            .help("Major or Minor."))
        .get_matches_from(args);

    let note_input: &str = matches.value_of("note").unwrap_or("C5");
    let scale_type: &str = matches.value_of("scale-type").unwrap_or("major");
    
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let notes = parse_csv("src/notes.csv");
    let root_note: Vec<Note> = notes.clone().into_iter()
        .filter(|n| n.note == note_input).collect();
    //println!("{:?}", build_scale(vec));
    let scale = build_scale(&root_note[0], scale_type);
    let match_notes = match_root(&notes, &note_input);

    match match_notes {
        Some(index) => {
            let scale_indices: Vec<usize> = if scale_type == "minor" {
                vec![0, 2, 3, 5, 7, 8, 10, 12]
            } else if scale_type == "major" {
                vec![0, 2, 4, 5, 7, 9, 11, 12]
            } else {
                Vec::new()
            };

            let scale_notes: Vec<String> = scale_indices
                .iter()
                .map(|&i| notes[index + i].note.clone())
                .collect();

            let scale_notes_str = scale_notes.join(" ");
            println!("{}", scale_notes_str);
        }
        None => println!("Note not found."),
    }

    play_scale(sink, scale);
}
