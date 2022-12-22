use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use rodio::{Decoder, OutputStream, source::Source};
use std::io::BufReader;
use std::fs::File;
fn f_n_int_from_string(_: &str) -> i32 { 4 }
fn main() {
    let stdin = stdin();
    //setting up stdout and going into raw mode
    let mut stdout = stdout().into_raw_mode().unwrap();
    //printing welcoming message, clearing the screen and going to left top corner with the cursor
    write!(stdout, r#"{}{}ctrl + q to exit, ctrl + h to print "Hello world!", alt + t to print "termion is cool""#, termion::cursor::Goto(1, 1), termion::clear::All)
            .unwrap();
    stdout.flush().unwrap();

    //detecting keydown events
    for c in stdin.keys() {
        //clearing the screen and going to top left corner
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All
        )
        .unwrap();

        //i reckon this speaks for itself
        match c.unwrap() {
            Key::Char(' ') => {
                // Get a output stream handle to the default physical sound device
                let (_stream, stream_handle) = OutputStream::try_default().unwrap();
                // Load a sound from a file, using a path relative to Cargo.toml
                let file = BufReader::new(File::open("./audio_files/kick.wav").unwrap());
                // Decode that sound file into a source
                let source = Decoder::new(file).unwrap();
                let o_duration = source.total_duration().unwrap();
                // println!("n_duration_ms: {:?}",f_n_int_from_string(&n_duration_ms));

                // Play the sound directly on the device
                stream_handle.play_raw(source.convert_samples());

                // The sound plays in a separate audio thread,
                // so we need to keep the main thread alive while it's playing.
                std::thread::sleep(o_duration);
                println!("kick");
            }
            Key::Ctrl('h') => println!("Hello world!"),
            Key::Ctrl('q') => break,
            Key::Alt('t') => println!("termion is cool"),
            _ => (),
        }

        stdout.flush().unwrap();
    }
}