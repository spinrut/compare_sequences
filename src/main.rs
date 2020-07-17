use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

/**
 * Given a file, read the first line as a reference sequence of characters
 * and, for each remaining line, compare the line to the reference sequence
 * and output a string of 0s and 1s, where a 0 in the nth position in a line
 * indicates that the nth characters match and a 1 indicates that they don't
 *
 * For lines longer than the reference sequence, ignore the extra characters
 * For lines shorter than the reference sequence, pad w/ 0s
 */
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
        line = line.trim().into();

        let ref_seq = line.clone();
        line.clear();

        while buf_reader.read_line(&mut line)? != 0 {
            // Remove linebreak
            line = line.trim().into();

            let diff: String = ref_seq.chars()
                .zip(line.drain(..))
                .map(|(ref_char, other_char)| if ref_char == other_char { '0' } else { '1' })
                .chain(std::iter::repeat('0')) // If given sequence is shorter than ref_seq, pad with 0.
                .take(ref_seq.len())
                .collect();

            println!("{}", diff);
        }

        Ok(())
    } else {
        println!("File is empty");
        Ok(())
    }
}
