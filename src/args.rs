use std::env;
use std::path::Path;
//use regex::Regex;

const DEFAULT_HEIGHT: u32 = 1024;
const DEFAULT_WIDTH: u32 = 1024;
const DEFAULT_ITERATIONS: u32 = 100;

const MAX_HEIGHT: u32 = 4096;
const MAX_WIDTH: u32 = 4096;

const MIN_HEIGHT: u32 = 128;
const MIN_WIDTH: u32 = 128;

//  Надо было clap юзать и не париться
pub fn parse_arguments() -> Result<(String, u32, u32, u32), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(String::from("Expected minimum two arguments!"));
    }

    let mut filepath = None;
    let mut width = None;
    let mut height = None;
    let mut iterations = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-f" | "--file" => {
                if filepath.is_some() {
                    return Err("File already specified!".to_string());
                }
                validate_filepath(&args[i + 1])?;
                filepath = Some(args[i + 1].clone());
                i += 2;
            }
            "-h" | "--height" => {
                if height.is_some() {
                    return Err("Height already specified!".to_string());
                }
                let h = args[i + 1]
                    .clone()
                    .parse::<u32>()
                    .map_err(|_e| String::from("Height not specified!"))?;
                validate_height(h)?;
                height = Some(h);
                i += 2;
            }
            "-w" | "--width" => {
                if width.is_some() {
                    return Err("Width already specified!".to_string());
                }
                let w = args[i + 1]
                    .clone()
                    .parse::<u32>()
                    .map_err(|_e| String::from("Width not specified!"))?;
                validate_width(w)?;
                width = Some(w);
                validate_width(width.unwrap().clone())?;
                i += 2;
            }
            "-i" | "--iterations" => {
                let iter = args[i + 1].clone().parse::<u32>().map_err(|_e| {
                    String::from(
                        "Iterations \
                not specified!",
                    )
                })?;
                iterations = Some(iter);
                i += 2;
            }
            arg => {
                return Err(format!("Unknown argument: {}", arg));
            }
        }
    }
    if width.is_none() {
        width = Some(DEFAULT_WIDTH);
    }
    if height.is_none() {
        height = Some(DEFAULT_HEIGHT);
    }
    if iterations.is_none() {
        iterations = Some(DEFAULT_ITERATIONS);
    }

    match filepath {
        Some(filepath) => Ok((
            filepath,
            width.unwrap(),
            height.unwrap(),
            iterations.unwrap(),
        )),
        None => Err(String::from("Filepath argument not provided")),
    }
}

fn validate_filepath(filepath: &str) -> Result<(), String> {
    /*let re = Regex::new(r"^(\w+)\.(\w+)$").unwrap();
    if !re.is_match(filepath) {
        return Err("Filepath argument is not valid!".to_string());
    }
    Ok(())*/
    let path = Path::new(filepath);

    if path.file_name().is_some() && path.extension().is_some() {
        Ok(())
    } else {
        Err("Filepath argument is not valid!".to_string())
    }
}

fn validate_width(width: u32) -> Result<(), String> {
    if width < MIN_WIDTH {
        let err_mess = format!("Width must be greater than {} pixels!", MIN_WIDTH);
        return Err(err_mess);
    }
    if width > MAX_WIDTH {
        let err_mess = format!("Width must be less than {} pixels!", MAX_WIDTH);
        return Err(err_mess);
    }
    Ok(())
}

fn validate_height(height: u32) -> Result<(), String> {
    if height < MIN_HEIGHT {
        let err_mess = format!("Height must be greater than {} pixels!", MIN_HEIGHT);
        return Err(err_mess);
    }
    if height > MAX_HEIGHT {
        let err_mess = format!("Height must be less than {} pixels!", MAX_HEIGHT);
        return Err(err_mess);
    }
    Ok(())
}