use rodio::{Decoder, OutputStream, Sink, source::Source};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::seq::SliceRandom;
use std::env;

pub struct Audio {
    pub sink: Arc<Mutex<Sink>>,
}

impl Audio {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        Self {
            sink: Arc::new(Mutex::new(sink)),
        }
    }

    pub fn play(&self, path: &str, duration: u64, volume: f32) {
        let audio_thread = {
            let path = path.to_string();
            thread::spawn(move || {
                let (_stream, stream_handle) = OutputStream::try_default().unwrap();
                let current_dir = env::current_dir().unwrap();
                let audio_path = current_dir.join(&path);
                let file = BufReader::new(File::open(audio_path.clone()).unwrap());
                let source = Decoder::new(file).unwrap();
                let source = source.amplify(volume);

                let sink = Sink::try_new(&stream_handle).unwrap();
                sink.append(source);

                std::thread::sleep(Duration::from_secs(duration));
                sink.stop();
            })
        };

        audio_thread.join().unwrap();
    }

    pub fn play_playlist(&self, paths: Vec<String>, loop_playlist: bool, random: bool, volume: f32) {
        let _ = thread::spawn(move || {
            loop {
                let mut playlist = paths.clone();
                if random {
                    let mut rng = rand::thread_rng();
                    playlist.shuffle(&mut rng);
                }

                for path in &playlist {
                    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
                    let file = BufReader::new(File::open(Path::new(path)).unwrap());
                    let source = Decoder::new(file).unwrap();
                    let source = source.amplify(volume);

                    let sink = Sink::try_new(&stream_handle).unwrap();
                    sink.append(source);
                    sink.sleep_until_end();
                }

                if !loop_playlist {
                    break;
                }
            }
        });
    }

    pub fn play_simple(path: &str) {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let file = BufReader::new(File::open(path).unwrap());
        let source = Decoder::new(file).unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        sink.append(source);
        sink.sleep_until_end();
    }
}

#[cfg(test)]
mod tests {
    use super::* ;

    #[test]
    fn test_audio_new() {
        let audio = Audio::new();
        assert!(audio.sink.lock().is_ok());
    }

    #[test]
    fn test_play() {
        let audio = Audio::new();
        let path = "src/sound_manager/sounds/guitar.mp3";
        let duration = 1;
        let volume = 0.5;
        audio.play(path, duration, volume);
    }

    #[test]
    fn test_simple() {
        let path = "src/sound_manager/sounds/guitar.mp3";
        Audio::play_simple(path);
    }

    #[test]
    fn test_playlist() {
        let audio = Audio::new();
        let paths = vec!["src/sound_manager/sounds/guitar.mp3".to_string(), "src/sound_manager/sounds/thunder.mp3".to_string()];
        audio.play_playlist(paths, false, false, 0.5);
        std::thread::sleep(Duration::from_secs(10));
    }
}