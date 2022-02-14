pub fn ask_audiofile_path() -> Option<String> {
    match tinyfiledialogs::open_file_dialog("Open", "null", Some((&["*.wav"], "Wav files"))) {
        Some(f) => Some(f),
        None => None,
    }
}
