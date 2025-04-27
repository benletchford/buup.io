use anyhow::{anyhow, Result};
use buup::{all_transformers, categorized_transformers, transformer_from_id, TransformerCategory};
use clap::{Arg, ArgAction, Command};
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;

fn main() -> Result<()> {
    // Create the base command with better formatting
    let mut app = Command::new("buup")
        .about("Text transformation utility belt")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg_required_else_help(true);

    // Add list command
    app = app.subcommand(
        Command::new("list")
            .about("List all available transformers")
            .display_order(1),
    );

    // Dynamically add a subcommand for each transformer
    for transformer in all_transformers() {
        let transformer_name = transformer.name().to_string();
        let transformer_desc = transformer.description().to_string();

        app = app.subcommand(
            Command::new(transformer.id())
                .about(format!("{} - {}", transformer_name, transformer_desc))
                .arg(
                    Arg::new("input")
                        .short('i')
                        .long("input")
                        .help("Input file (stdin if not specified)")
                        .value_name("FILE")
                        .num_args(1),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .help("Output file (stdout if not specified)")
                        .value_name("FILE")
                        .num_args(1),
                )
                .arg(
                    Arg::new("text")
                        .help("Input text provided directly")
                        .action(ArgAction::Append)
                        .trailing_var_arg(true),
                ),
        );
    }

    // Parse the arguments
    let matches = app.get_matches();

    // Process the command
    match matches.subcommand() {
        Some(("list", _)) => list_transformers(),
        Some((command_name, sub_matches)) => {
            // Check if the command name matches a transformer ID
            if let Ok(transformer) = transformer_from_id(command_name) {
                let input = sub_matches.get_one::<String>("input").map(PathBuf::from);
                let output = sub_matches.get_one::<String>("output").map(PathBuf::from);
                let text = sub_matches
                    .get_many::<String>("text")
                    .map(|v| v.cloned().collect())
                    .unwrap_or_default();

                transform(transformer, input, output, text)
            } else {
                Err(anyhow!("Unknown command: {}", command_name))
            }
        }
        _ => Err(anyhow!(
            "No command specified. Try 'buup list' to see available transformers."
        )),
    }
}

fn list_transformers() -> Result<()> {
    println!("Available transformers:");

    // Get transformers categorized by the library function
    let categories = categorized_transformers();

    let encoders = categories.get(&TransformerCategory::Encoder).unwrap();
    let decoders = categories.get(&TransformerCategory::Decoder).unwrap();
    let formatters = categories.get(&TransformerCategory::Formatter).unwrap();
    let cryptography = categories.get(&TransformerCategory::Crypto).unwrap();
    let compression = categories.get(&TransformerCategory::Compression).unwrap();
    let others = categories.get(&TransformerCategory::Other).unwrap();

    // Print groups with better formatting
    if !encoders.is_empty() {
        println!("\nENCODERS:");
        for t in encoders {
            println!("  {:<15} - {}", t.id(), t.description());
        }
    }

    if !decoders.is_empty() {
        println!("\nDECODERS:");
        for t in decoders {
            println!("  {:<15} - {}", t.id(), t.description());
        }
    }

    if !formatters.is_empty() {
        println!("\nFORMATTERS:");
        for t in formatters {
            println!("  {:<15} - {}", t.id(), t.description());
        }
    }

    if !cryptography.is_empty() {
        println!("\nCRYPTOGRAPHY:");
        for t in cryptography {
            println!("  {:<15} - {}", t.id(), t.description());
        }
    }

    if !compression.is_empty() {
        println!("\nCOMPRESSION:");
        for t in compression {
            println!("  {:<15} - {}", t.id(), t.description());
        }
    }

    if !others.is_empty() {
        println!("\nOTHERS:");
        for t in others {
            println!("  {:<15} - {}", t.id(), t.description());
        }
    }

    // Usage examples
    println!("\nEXAMPLES:");
    println!("  buup base64encode \"Hello, world!\"     # Encode text directly");
    println!("  buup urldecode -i encoded.txt         # Decode from file");
    println!("  echo \"Hello\" | buup hexencode         # Pipe from stdin");

    Ok(())
}

fn transform(
    transformer: &dyn buup::Transform,
    input_path: Option<PathBuf>,
    output_path: Option<PathBuf>,
    text_args: Vec<String>,
) -> Result<()> {
    // Read input based on priority:
    // 1. Text provided as arguments
    // 2. Input file specified by path
    // 3. Stdin
    let input = if !text_args.is_empty() {
        text_args.join(" ")
    } else {
        read_input(input_path)?
    };

    // Transform the input
    let output = transformer
        .transform(&input)
        .map_err(|e| anyhow!("Transformation error: {}", e))?;

    // Write output
    write_output(output_path, output)?;

    Ok(())
}

fn read_input(input_path: Option<PathBuf>) -> Result<String> {
    match input_path {
        Some(path) => {
            let mut file = File::open(path)?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            Ok(content)
        }
        None => {
            // Check if stdin has data available
            let stdin = io::stdin();
            let mut stdin_handle = stdin.lock();
            let mut content = String::new();

            // We use read_to_string which will read until EOF
            stdin_handle.read_to_string(&mut content)?;

            Ok(content)
        }
    }
}

fn write_output(output_path: Option<PathBuf>, content: String) -> Result<()> {
    match output_path {
        Some(path) => {
            let mut file = File::create(path)?;
            file.write_all(content.as_bytes())?;
            Ok(())
        }
        None => {
            print!("{}", content);
            io::stdout().flush()?;
            Ok(())
        }
    }
}
