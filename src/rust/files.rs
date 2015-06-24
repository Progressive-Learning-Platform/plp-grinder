use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;

pub fn read_in(file_path: &str) -> String
{
    let path = Path::new(file_path);
    let display = path.display();

    let mut file = match File::open(&path)
    {
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    let mut file_as_string = String::new();
    match file.read_to_string(&mut file_as_string) {
        Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
        Ok(_) => {},
    }

    file_as_string
}

pub fn dump(file_path: &str, data: String)
{
    let path = Path::new(file_path);
    let display = path.display();

    let mut file = match File::create(&path)
    {
        Err(why) => panic!("couldn't create {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    match file.write_all(data.as_bytes())
    {
        Err(why) => {panic!("couldn't write to {}: {}", display, Error::description(&why))},
        Ok(_) => println!("successfully output to {}", display),
    }
}
