use buup::{categorized_transformers, transformer_from_id, Transform, TransformerCategory};
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;

fn print_usage() {
    println!("buup - Text transformation utility belt");
    println!();
    println!("USAGE:");
    println!("  buup [COMMAND] [OPTIONS] [INPUT]");
    println!();
    println!("COMMANDS:");
    println!("  list               List all available transformers");
    println!("  [transformer_id]   Run the specified transformer");
    println!();
    println!("OPTIONS:");
    println!("  -i, --input FILE   Input file (stdin if not specified)");
    println!("  -o, --output FILE  Output file (stdout if not specified)");
    println!("  -h, --help         Show this help message");
    println!("  -v, --version      Show version information");
    println!();
    println!("Run 'buup list' to see all available transformers");
}

fn print_version() {
    println!("buup {}", env!("CARGO_PKG_VERSION"));
}

fn list_transformers() -> Result<(), String> {
    println!("Available transformers:");

    // Get transformers categorized by the library function
    let categories = categorized_transformers();

    let encoders = categories.get(&TransformerCategory::Encoder).unwrap();
    let decoders = categories.get(&TransformerCategory::Decoder).unwrap();
    let formatters = categories.get(&TransformerCategory::Formatter).unwrap();
    let cryptography = categories.get(&TransformerCategory::Crypto).unwrap();
    let compression = categories.get(&TransformerCategory::Compression).unwrap();
    let colors = categories.get(&TransformerCategory::Color).unwrap();
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

    if !colors.is_empty() {
        println!("\nCOLORS:");
        for t in colors {
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

fn read_input(input_path: Option<PathBuf>) -> Result<String, String> {
    match input_path {
        Some(path) => {
            let mut file =
                File::open(path).map_err(|e| format!("Failed to open input file: {}", e))?;
            let mut content = String::new();
            file.read_to_string(&mut content)
                .map_err(|e| format!("Failed to read input file: {}", e))?;
            Ok(content)
        }
        None => {
            // Check if stdin has data available
            let stdin = io::stdin();
            let mut stdin_handle = stdin.lock();
            let mut content = String::new();

            // We use read_to_string which will read until EOF
            stdin_handle
                .read_to_string(&mut content)
                .map_err(|e| format!("Failed to read from stdin: {}", e))?;

            Ok(content)
        }
    }
}

fn write_output(output_path: Option<PathBuf>, content: String) -> Result<(), String> {
    match output_path {
        Some(path) => {
            let mut file =
                File::create(path).map_err(|e| format!("Failed to create output file: {}", e))?;
            file.write_all(content.as_bytes())
                .map_err(|e| format!("Failed to write to output file: {}", e))?;
            Ok(())
        }
        None => {
            print!("{}", content);
            io::stdout()
                .flush()
                .map_err(|e| format!("Failed to flush stdout: {}", e))?;
            Ok(())
        }
    }
}

fn transform(
    transformer: &dyn Transform,
    input_path: Option<PathBuf>,
    output_path: Option<PathBuf>,
    text_args: Vec<String>,
) -> Result<(), String> {
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
        .map_err(|e| format!("Transformation error: {}", e))?;

    // Write output
    write_output(output_path, output)?;

    Ok(())
}

fn parse_args(args: Vec<String>) -> Result<(), String> {
    if args.len() <= 1 {
        print_usage();
        return Ok(());
    }

    let command = &args[1];

    if command == "list" {
        return list_transformers();
    } else if command == "--help" || command == "-h" {
        print_usage();
        return Ok(());
    } else if command == "--version" || command == "-v" {
        print_version();
        return Ok(());
    }

    // Check if the command name matches a transformer ID
    match transformer_from_id(command) {
        Ok(transformer) => {
            let mut input_path = None;
            let mut output_path = None;
            let mut text_args = Vec::new();
            let mut i = 2;

            while i < args.len() {
                if args[i] == "-i" || args[i] == "--input" {
                    if i + 1 >= args.len() {
                        return Err("Missing input file path".to_string());
                    }
                    input_path = Some(PathBuf::from(&args[i + 1]));
                    i += 2;
                } else if args[i] == "-o" || args[i] == "--output" {
                    if i + 1 >= args.len() {
                        return Err("Missing output file path".to_string());
                    }
                    output_path = Some(PathBuf::from(&args[i + 1]));
                    i += 2;
                } else if args[i] == "-h" || args[i] == "--help" {
                    print_usage();
                    return Ok(());
                } else {
                    // Collect all remaining args as text input
                    text_args.extend(args[i..].iter().cloned());
                    break;
                }
            }

            transform(transformer, input_path, output_path, text_args)
        }
        Err(_) => Err(format!(
            "Unknown transformer: {}. Run 'buup list' to see available transformers.",
            command
        )),
    }
}

pub fn main() {
    let args: Vec<String> = std::env::args().collect();

    match parse_args(args) {
        Ok(_) => std::process::exit(0),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1)
        }
    }
}
