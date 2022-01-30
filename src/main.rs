mod audio_file;
mod audio_player;
mod audio_stream;
mod audio_track;
mod ui;
mod wavreader;

const FILE_PATH1: &str = "./testsamples/mel1_i24.wav";
const FILE_PATH2: &str = "./testsamples/sub_i24.wav";

fn main() {
    let app = ui::app::App::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
