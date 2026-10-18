use std::io;
use std::path::PathBuf;

use pywire_python_ast::PBDecoder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (path, threads) = parse_arguments()?;
    let decoder = PBDecoder::new(path).with_threads(threads);
    let modules = decoder.decode_dir()?;
    let workers = decoder.threads().min(modules.len().max(1));

    println!(
        "decoded {} Python AST modules using {} thread(s)",
        modules.len(),
        workers
    );
    Ok(())
}

fn parse_arguments() -> Result<(PathBuf, usize), Box<dyn std::error::Error>> {
    let mut arguments = std::env::args().skip(1);
    let mut threads = 1;
    let mut path = None;

    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--parallel" => {
                threads = std::thread::available_parallelism()?.get();
            }
            "--threads" | "-j" => {
                let value = arguments.next().ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "missing thread count")
                })?;
                threads = value.parse::<usize>().map_err(|error| {
                    io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("invalid thread count '{value}': {error}"),
                    )
                })?;
                if threads == 0 {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "thread count must be at least 1",
                    )
                    .into());
                }
            }
            "--help" | "-h" => {
                println!(
                    "Usage: pywire-python-ast-decode [PATH] [--parallel | --threads N]\n\n\
                     PATH defaults to ./ir_out."
                );
                std::process::exit(0);
            }
            _ => {
                if argument.starts_with('-') || path.is_some() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("unexpected argument '{argument}'"),
                    )
                    .into());
                }
                path = Some(PathBuf::from(argument));
            }
        }
    }

    Ok((path.unwrap_or_else(|| PathBuf::from("ir_out")), threads))
}
