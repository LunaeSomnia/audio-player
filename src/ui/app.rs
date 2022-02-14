use eframe::{egui, epi};

use std::sync::{Arc, Mutex};

use crate::audio::audiofile::AudioFile;
use crate::audio::audiostream::AudioStream;
use crate::audio_player::AudioPlayer;
use crate::audio_track::AudioTrack;

pub struct AppCtx {
    playing: bool,
    slider1: f32,
    slider2: f32,
}

pub struct App {
    stream: AudioStream,
    player: Arc<Mutex<AudioPlayer>>,
    ctx: Arc<Mutex<AppCtx>>,
}

impl Default for App {
    fn default() -> Self {
        let (stream, prod) = AudioStream::new();
        let mut player = AudioPlayer::new(prod);

        stream.stop().unwrap();

        player.attach(AudioTrack::new(
            "melody",
            AudioFile::open(crate::FILE_PATH1).unwrap(),
        ));
        player.attach(AudioTrack::new(
            "sub",
            AudioFile::open(crate::FILE_PATH2).unwrap(),
        ));

        let ph = Arc::new(Mutex::new(player));
        let pe = ph.clone();

        let context = Arc::new(Mutex::new(AppCtx {
            playing: false,
            slider1: 0.8,
            slider2: 0.8,
        }));
        let ctx = context.clone();

        std::thread::spawn(move || loop {
            let ctx = ctx.lock().unwrap();
            let mut player_data = pe.lock().unwrap();

            player_data.get("melody").unwrap().set_volume(ctx.slider1);
            player_data.get("sub").unwrap().set_volume(ctx.slider2);

            // This needs to change in the future.
            // Ideally we don't need to stop inputting stuff into the buffer,
            //  instead, we stop (or dont send) from the stream itself
            if ctx.playing {
                player_data.advance();
            }
        });

        Self {
            stream,
            player: ph,
            ctx: context,
        }
    }
}

impl epi::App for App {
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame) {
        let mut d_ctx = self.ctx.lock().unwrap();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|h| {
                if h.button("Play").clicked() {
                    d_ctx.playing = true;
                }

                if h.button("Pause").clicked() {
                    d_ctx.playing = false;
                }
            });

            ui.horizontal(|h| {
                h.vertical(|v| {
                    v.add(egui::Label::new("Melody"));
                    v.add(egui::Slider::new(&mut d_ctx.slider1, 0.0..=1.0));
                });

                h.vertical(|v| {
                    v.add(egui::Label::new("Sub"));
                    v.add(egui::Slider::new(&mut d_ctx.slider2, 0.0..=1.0));
                });
            });
        });
    }

    fn name(&self) -> &str {
        "Simple Player"
    }

    fn on_exit(&mut self) {
        self.stream.stop().unwrap();
    }
}
