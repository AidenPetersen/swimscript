use std::{fs, io};
use clap::{ErrorKind, IntoApp, Parser};
use parser;

#[derive(Parser)]
#[clap(name = "swimscript")]
#[clap(author = "Aiden Petersen")]
#[clap(about = "A programming language for swim practices.", long_about = None)]
struct Args {
    /// Name of .swim file
    input_file: Option<String>,

    /// Name of the output file
    output_file: Option<String>,

    /// Format to compile to
    #[clap(short, long, default_value = "json")]
    format: String,
}

fn write_output(data: String, args: Args) -> io::Result<()> {
    fs::write(args.output_file.unwrap_or_else(|| {
        let mut app = Args::into_app();
        app.error(
            ErrorKind::MissingRequiredArgument,
            "OUTPUT_FILE is required",
        ).exit()
    }), data)
}

const FORMATS: (&str, &str) = ("json", "ron");

fn get_input(args: &Args) -> String {
    let file = args
        .input_file
        .as_deref()
        // Error if missing args
        .unwrap_or_else(|| {
            let mut app = Args::into_app();
            app.error(
                ErrorKind::MissingRequiredArgument,
                "INPUT_FILE is required",
            ).exit()
        });

    let mut data = fs::read_to_string(file).unwrap_or_else(|_| {
        let mut app = Args::into_app();
        // Error if it doesn't exist
        app.error(
            ErrorKind::ValueValidation,
            "INPUT_FILE is not a valid file",
        ).exit()
    });
    data
}

fn main() {
    // Setup CLI
    let args: Args = Args::parse();

    let data = get_input(&args);


    match args.format.to_lowercase().as_str() {
        "json" => {
            let json = serde_json::to_string(&result).unwrap();
            let _ = write_output(json, args);
        }
        "ron" => {
            let ron = ron::to_string(&result).unwrap();
            let _ = write_output(ron, args);
        }
        str => {
            let mut app = Args::into_app();
            app.error(
                ErrorKind::ValueValidation,
                format!("{} is not a valid format type, currently\n{:?}\nare supported",
                        str, FORMATS),
            ).exit()
        }
    }
}