use std::time::Duration;
use rodio::{OutputStream, Sink};
use rodio::source::{SineWave, Source};

fn play_scale_array(sink: Sink, scale: [f32; 8]) {
    //Add a dumny source for the sake of the example
    for x in scale {
        let note = SineWave::new(x).take_duration(Duration::from_secs_f32(0.5)).amplify(0.20);
        sink.append(note);
        sink.sleep_until_end();
    }
}

fn play_scale_vector(sink: Sink, scale: Vec<f32>) {
    for step in scale {
        let note = SineWave::new(step).take_duration(Duration::from_secs_f32(0.5)).amplify(0.20);
        sink.append(note);
        sink.sleep_until_end();
    }
}

fn build_scale(root: f32) -> Vec<f32> {
    //major scale: whole whole half whole whole whole half 
    //minor scale: whole half whole whole whole half whole whole
    //C D E F G A B C
    // A B C D E F G A
    //TODO integrate note/half not notation to build out major and minor scale, work in to calculationg
    let mut scale = vec![0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0];
    let mult: f32 = 1.0595;
    let mut tuning: f32 = root;
    for i in scale.iter_mut() {
      *i = tuning;
      tuning *= mult;
    }
    scale
}


fn main() {
    let tuning: f32 = 440.0; //A4
    let mult: f32 = 1.0595;
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let c_maj: [f32; 8] = [523.25, 587.33, 659.25,698.46,783.99,880.0,987.77,1046.50];

    
    //println!("{:?}", build_scale(vec));
    let scale_test = build_scale(tuning);
    play_scale_vector(sink, scale_test);
    //play_scale_array(sink, c_maj)
}
