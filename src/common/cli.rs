use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about="Nano Image Server is a tiny, blazingly fast service to serve images with support for basic image operation on fly.", long_about = None)]
pub struct Args {
    // --------------------------------------------
    // Port and URL Configurations
    /// Defines the port the app is hosted on
    #[arg(long, short, default_value_t = 8000)]
    pub port: u16,

    /// Base url of the app (needed when self hosting)
    #[arg(long, short)]
    pub base_url: Option<String>,

    // --------------------------------------------
    // Security configuration
    /// Enables TLS. Requires certificates in cert folder
    #[arg(long, short, default_value_t = false)]
    tls: bool,

    /// Enables Security Middleware. Enabled by default, Disabling bypasses the middleware entirely.
    #[arg(long, short, default_value_t = false)]
    security_middleware: bool,

    // --------------------------------------------
    // Plugins support
    /// Enable Plugin support
    #[arg(long, short, default_value_t = false)]
    allow_plugins: bool,

    // --------------------------------------------
    // Caching configuration
    /// Toggle for Caching. Set to false by default and allows caching.
    #[arg(long, short, default_value_t = false)]
    no_cache: bool,

    /// Limit for memory cache in Megabytes.
    #[arg(long, short, default_value_t = 1024)]
    mem_cache_limit: u32,

    /// Limit for storage based caching in Megabytes.
    #[arg(long, short, default_value_t = 1024*4)]
    cache_limit: u32,
    // --------------------------------------------
}
