use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if let Some(filename) = args.get(1) {
        File::open(filename).and_then(read_file)
    } else {
        println!("Usage: compare_sequences.exe name_of_file");
        Ok(())
    }
}

fn read_file(file: File) -> std::io::Result<()> {
    let mut buf_reader = BufReader::new(file);
    let mut line = String::new();
    let read_chars = buf_reader.read_line(&mut line)?;
    if read_chars != 0 {
        // Remove linebreak
        line.pop();

        let ref_seq = line.clone();
        line.clear();

        while buf_reader.read_line(&mut line)? != 0 {
            // Remove linebreak
            line.pop();

            if ref_seq.len() != line.len() {
                println!("Sequences are of different lengths.");
                println!("Reference sequence: {}", ref_seq);
                println!("Other sequence:     {}", line);
            }

            let diff: String = ref_seq.chars()
                .zip(line.drain(..))
                .map(|(ref_char, other_char)| if ref_char == other_char { '0' } else { '1' })
                .collect();

            println!("{}", diff);
        }

        Ok(())
    } else {
        println!("File is empty");
        Ok(())
    }
}
