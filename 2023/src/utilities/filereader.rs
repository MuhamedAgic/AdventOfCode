use std::{fs::{self, File}, 
          io::{self, BufRead}, 
          path::Path};

pub fn get_file_as_str(path: &str) -> String 
{
    let data = fs::read_to_string(path).expect(format!("Unable to read file '{}'", path).as_str());
    return data;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}