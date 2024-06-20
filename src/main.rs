use colored::*;
use regex::Regex;
use std::env;
use std::fs;

struct Arguments<'a> {
    pattern: &'a String,
    replace: &'a String,
    input_file: &'a String,
    output_file: &'a String,
}

/// This function parses the command line arguments and returns a struct containing the parsed arguments.
///
/// # Arguments
///
/// * `argc` - An unsigned integer representing the number of command line arguments.
/// * `argv` - A reference to a vector of strings representing the command line arguments.
///
/// # Returns
///
/// * `Arguments` - A struct containing the parsed arguments.
///
/// # Panics
///
/// If the number of command line arguments is not equal to 5, the function will panic with an error message.
fn parse_args(argc: usize, argv: &Vec<String>) -> Arguments {
    // Check if the number of command line arguments is correct
    if argc != 5 {
        // Print an error message and exit the program
        eprintln!("{} wrong number of arguments!", "ERROR".red());
        std::process::exit(1);
    }

    // Create a new Arguments struct and return it
    Arguments {
        pattern: &argv[1],  // The pattern to be replaced
        replace: &argv[2],  // The replacement text
        input_file: &argv[3],  // The input file path
        output_file: &argv[4],  // The output file path
    }
}

/// This function reads the content of the input file and returns it as a string.
///
/// # Arguments
///
/// * `args` - A reference to the `Arguments` struct containing the input file path.
///
/// # Returns
///
/// * `String` - The content of the input file as a string.
///
/// # Errors
///
/// If an error occurs while reading the file, an error message is printed to the standard error stream,
/// and the program exits with a status code of 1.
fn read(args: &Arguments) -> String {
    let data = match fs::read_to_string(&args.input_file) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("{} Failed to read from file {}", "ERROR".red(), &args.input_file);
            std::process::exit(1);
        }
    };

    data
}

/// This function replaces all occurrences of a pattern in a given string with a replacement.
///
/// # Arguments
///
/// * `search_target` - A reference to a string representing the pattern to be replaced.
/// * `replacement` - A reference to a string representing the replacement text.
/// * `data` - A reference to a string representing the input data.
///
/// # Returns
///
/// * `Result<String, regex::Error>` - A Result containing the replaced string if successful, or a regex::Error if the pattern is invalid.
///
/// # Errors
///
/// If the pattern is invalid, a regex::Error is returned.
fn replace(search_target: &str, replacement: &str, data: &str) -> Result<String, regex::Error> {
    // Create a new Regex object from the search_target pattern
    let regex = Regex::new(search_target)?;

    // Replace all occurrences of the pattern in the data with the replacement text
    Ok(regex.replace_all(data, replacement).to_string())
}

/// Writes the replaced data to the output file.
///
/// # Arguments
///
/// * `args` - A reference to the `Arguments` struct containing the input and output file paths.
/// * `data` - A reference to the string containing the replaced data.
///
/// # Errors
///
/// If an error occurs while writing to the output file, an error message is printed to the standard error stream,
/// and the program exits with a status code of 1.
fn write(args: &Arguments, data: &String) {
    match fs::write(&args.output_file, &data) {
        Ok(_) => {},
        Err(_) => {
            eprintln!("{} Failed to write to file {}", "ERROR".red(), &args.output_file);
            std::process::exit(1);
        }
    }
}

/// This function runs the main logic of the program.
/// It parses the command line arguments, reads the input file, replaces the pattern with the replacement,
/// and writes the result to the output file.
///
/// # Arguments
///
/// * `argc` - The number of command line arguments.
/// * `argv` - A vector of strings representing the command line arguments.
fn run(argc: usize, argv: Vec<String>) {
    // Print the number of command line arguments
    println!("Argc: {}", argc);

    // Print the command line arguments
    println!("Argv: {:?}", argv);

    // Parse the command line arguments
    let args = parse_args(argc, &argv);

    // Read the input file
    let data = read(&args);

    // Replace the pattern with the replacement in the data
    let replaced_data = match replace(&args.pattern, &args.replace, &data) {
        Ok(d) => d,
        Err(_) => {
            // Exit the program if an error occurs during replacement
            std::process::exit(1);
        }
    };

    // Write the replaced data to the output file
    write(&args, &replaced_data);
}

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();

    run(argc, argv);
}
