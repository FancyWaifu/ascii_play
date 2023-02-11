use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};

struct file
{
    filename: String,
}

impl file
{
    fn create(&self)
    {
        File::create(&self.filename).expect("Failed to create file");
    }

    fn read_file(&self)
    {
        let mut a = File::open(&self.filename).expect("Could not open file. Does it exist?");
        let mut content = String::new();
        a.read_to_string(&mut content);
        println!("{}", content);
    }

    fn exist(&self)
    {
        let a = File::open(&self.filename);
        match a 
        {
            Ok(a) => {
                println!("File exist");
            },
            Err(e) => {
                println!("File does not exist");
            },
        }
    }

    fn read_lines(&self)
    {
        let a = File::open(&self.filename);
        match a 
        {
            Ok(a) => {
                let reader = BufReader::new(a);
                for (index, line) in reader.lines().enumerate() {
                    let line = line.unwrap();
                    println!("{}. {}", index + 1, line);
                }
            },
            Err(e) => {
                println!("Could not open file. Does it exist?");
            },
        }
    }
}

fn main() 
{
    let FILE =  file {
        filename: String::from("hello.asciiplay"),
    };
    FILE.read_lines();
}
