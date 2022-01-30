use audio_file::AudioFile;
use audio_player::AudioPlayer;
use audio_track::AudioTrack;

mod audio_file;
mod audio_player;
mod audio_stream;
mod audio_track;
mod mixer;
mod wavreader;

const FILE_PATH1: &str = "./testsamples/own/mel1_i24.wav";
const FILE_PATH2: &str = "./testsamples/own/silence_i24.wav";

fn main() {
    let mut x = vec![];

    x.push(2);

    x.remove(0);

    let (stream, prod) = audio_stream::AudioStream::new();
    let mut player = AudioPlayer::new(prod);

    stream.play().unwrap();

    player.attach(AudioTrack::new(AudioFile::open(FILE_PATH1).unwrap()));
    player.attach(AudioTrack::new(AudioFile::open(FILE_PATH2).unwrap()));

    loop {
        player.advance();
    }

    stream.stop().unwrap();
}

// // Testing grounds using i24 44100hz .wav file
// fn main2() {
//     // Ring buffer creation

//     let ring_buf: rb::SpscRb<f32> = rb::SpscRb::new(4 * 1024);
//     let (pr, cn) = (ring_buf.producer(), ring_buf.consumer());

//     // Setting up the file reader and writer

//     let mut reader1 = WavReader::open(FILE_PATH1).unwrap();
//     let mut reader2 = WavReader::open(FILE_PATH2).unwrap();

//     println!("{}", reader1.duration() / reader1.spec().channels as u32);
//     println!("{}", reader2.duration() / reader1.spec().channels as u32);

//     let mut iterator1 = reader1.samples::<i32>();
//     let mut iterator2 = reader2.samples::<i32>();

//     // Stream setup

//     let dev = cpal::default_host().default_output_device().unwrap();
//     let conf = dev.default_output_config().unwrap();
//     let str = dev
//         .build_output_stream(
//             &conf.config(),
//             move |d: &mut [f32], _| {
//                 let written = cn.read(d).unwrap_or(0);
//                 d[written..].iter_mut().for_each(|s| *s = 0.0);
//             },
//             |e| {
//                 panic!("{}", e);
//             },
//         )
//         .unwrap();

//     // Filling the producer

//     thread::sleep(Duration::from_millis(500)); // If we don't sleep for a bit, the start 'pops' a bit

//     let mut playing = false;
//     loop {
//         let new_sample1 = match iterator1.next() {
//             Some(x) => x.unwrap() as f32 / 0x7FFFFF as f32,
//             None => break,
//         };

//         let new_sample2 = match iterator2.next() {
//             Some(x) => x.unwrap() as f32 / 0x7FFFFF as f32,
//             None => break,
//         };

//         // If the buffer is full, play the stream
//         if pr.write_blocking(&[new_sample1 + new_sample2]).is_none() && !playing {
//             str.play().unwrap();
//             playing = true;
//         };
//     }

//     // End

//     str.pause().unwrap();
// }
