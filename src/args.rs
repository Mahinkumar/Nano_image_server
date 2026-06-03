#[cfg(feature = "tls")]
use std::path::PathBuf;
pub struct Args {
    pub port: u16,
    pub base_url: Option<String>,
    #[cfg(feature = "tls")]
    pub cert_path: Option<PathBuf>,
    #[cfg(feature = "cache")]
    pub cache_capacity: usize,
}

impl Args {
    pub fn parse() -> Args {
        let mut port: u16 = 8000;
        let mut base_url: Option<String> = None;
        #[cfg(feature = "tls")]
        let mut cert_path: Option<PathBuf> = None;
        #[cfg(feature = "cache")]
        let mut cache_capacity: usize = 100;

        let mut args = std::env::args().skip(1); // skip the binary name

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-h" | "--help" => {
                    print_help();
                    std::process::exit(0);
                }
                "-V" | "--version" => {
                    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
                    std::process::exit(0);
                }
                "-p" | "--port" => {
                    port = next_value(&mut args, "--port")
                        .parse()
                        .unwrap_or_else(|_| fail("--port expects a number 0-65535"));
                }
                "-b" | "--base-url" => {
                    base_url = Some(next_value(&mut args, "--base-url"));
                }
                #[cfg(feature = "tls")]
                "-c" | "--cert-path" => {
                    cert_path = Some(PathBuf::from(next_value(&mut args, "--cert-path")));
                }
                #[cfg(feature = "cache")]
                "--cache-capacity" => {
                    cache_capacity = next_value(&mut args, "--cache-capacity")
                        .parse()
                        .unwrap_or_else(|_| fail("--cache-capacity expects a number"));
                }
                other => fail(&format!("unknown argument: {other}")),
            }
        }

        Args {
            port,
            base_url,
            #[cfg(feature = "tls")]
            cert_path,
            #[cfg(feature = "cache")]
            cache_capacity,
        }
    }
}

fn next_value(args: &mut impl Iterator<Item = String>, flag: &str) -> String {
    args.next()
        .unwrap_or_else(|| fail(&format!("{flag} expects a value")))
}

fn fail(msg: &str) -> ! {
    eprintln!("error: {msg}\n");
    print_help();
    std::process::exit(2);
}

fn print_help() {
    eprintln!(
        "\
{name} {version}
A tiny image server.

USAGE:
    {name} [OPTIONS]

OPTIONS:
    -p, --port <PORT>          Port to listen on [default: 8000]
    -b, --base-url <URL>       Base url where the app is hosted [default: localhost]",
        name = env!("CARGO_PKG_NAME"),
        version = env!("CARGO_PKG_VERSION"),
    );
    #[cfg(feature = "tls")]
    eprintln!("    -c, --cert-path <DIR>      Folder containing cert.pem and key.pem (PEM format)");
    #[cfg(feature = "cache")]
    eprintln!("        --cache-capacity <N>   Number of images to cache [default: 100]");
    eprintln!(
        "    -h, --help                 Print help\n    -V, --version              Print version"
    );
}
