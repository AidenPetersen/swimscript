use std::{fs, io};
use clap::{ErrorKind, IntoApp, Parser};

mod parser;


#[derive(Parser)]
#[clap(name = "swimscript")]
#[clap(author = "Aiden Petersen")]
#[clap(about = "A programming language for swim practices.", long_about = None)]
struct Args {
    /// Name of .swim file
    input_file: Option<String>,

    /// Name of the output file
    output_file: Option<String>,

    /// Verbose
    #[clap(short, long)]
    verbose: bool,

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
    data.push('\n');
    data
}

fn main() {
    // Setup CLI
    let args: Args = Args::parse();

    let data = get_input(&args);

    let parsed = parser::parser(data.as_str());
    if parsed.is_err() {
        println!("Error: Could not parse file");
        println!("{:#?}", parsed.as_ref().err())
    } else if args.verbose {
        println!("Parsed result: ");
        println!("{:#?}", parsed.as_ref().unwrap());
    }
    let result = parsed.unwrap().1;

    match args.format.to_lowercase().as_str() {
        "json" => {
            let json = serde_json::to_string(&result).unwrap();
            let _ = write_output(json, args);
        }
        "ron" => {
            let ron = ron::to_string(&result).unwrap();
            let _ = write_output(ron, args);

        }
        _ => {
            println!("Error: Not a valid format.");
            println!("Currently supported formats are:");
            println!("json, ron");
        }
    }
}