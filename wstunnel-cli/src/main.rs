use clap::Parser;
use serde::Deserialize;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;
use std::str::FromStr;
use tracing::warn;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::filter::Directive;
use wstunnel::LocalProtocol;
use wstunnel::config::{Client, Server};
use wstunnel::executor::DefaultTokioExecutor;
use wstunnel::{run_client, run_server};

#[cfg(feature = "jemalloc")]
use tikv_jemallocator::Jemalloc;

#[cfg(feature = "jemalloc")]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

/// Use Websocket or HTTP2 protocol to tunnel {TCP,UDP} traffic
/// wsTunnelClient <---> wsTunnelServer <---> RemoteHost
#[derive(clap::Parser, Debug)]
#[command(author, version, about, verbatim_doc_comment, long_about = None)]
pub struct Wstunnel {
    #[command(subcommand)]
    commands: Commands,

    /// Path to config file (YAML format)
    /// Config file can contain 'client' and/or 'server' sections
    /// CLI arguments take precedence over config file values
    #[arg(long, global = true, value_name = "FILE_PATH", verbatim_doc_comment)]
    config: Option<std::path::PathBuf>,

    /// Disable color output in logs
    #[arg(long, global = true, verbatim_doc_comment, env = "NO_COLOR")]
    no_color: Option<String>,

    /// *WARNING* The flag does nothing, you need to set the env variable *WARNING*
    /// Control the number of threads that will be used.
    /// By default, it is equal the number of cpus
    #[arg(
        long,
        global = true,
        value_name = "INT",
        verbatim_doc_comment,
        env = "TOKIO_WORKER_THREADS"
    )]
    nb_worker_threads: Option<u32>,

    /// Control the log verbosity. i.e: TRACE, DEBUG, INFO, WARN, ERROR, OFF
    /// for more details: https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#example-syntax
    #[arg(
        long,
        global = true,
        value_name = "LOG_LEVEL",
        verbatim_doc_comment,
        env = "RUST_LOG",
        default_value = "INFO"
    )]
    log_lvl: String,
}

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    Client(Box<Client>),
    Server(Box<Server>),
}

#[derive(Debug, Deserialize)]
struct ConfigFile {
    #[serde(default)]
    client: Option<Client>,
    #[serde(default)]
    server: Option<Server>,
}

fn load_config_file(path: &Path) -> anyhow::Result<ConfigFile> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let config: ConfigFile = serde_yaml::from_reader(reader)?;
    Ok(config)
}

fn merge_client_config(mut cli: Client, file_config: Option<Client>) -> Client {
    let Some(file) = file_config else {
        return cli;
    };
    
    // Merge config: CLI args override config file
    // Only override if CLI value is at default
    if cli.local_to_remote.is_empty() {
        cli.local_to_remote = file.local_to_remote;
    }
    if cli.remote_to_local.is_empty() {
        cli.remote_to_local = file.remote_to_local;
    }
    if cli.socket_so_mark.is_none() {
        cli.socket_so_mark = file.socket_so_mark;
    }
    if cli.connection_min_idle == 0 {
        cli.connection_min_idle = file.connection_min_idle;
    }
    if cli.connection_retry_max_backoff == std::time::Duration::from_secs(300) {
        cli.connection_retry_max_backoff = file.connection_retry_max_backoff;
    }
    if cli.reverse_tunnel_connection_retry_max_backoff == std::time::Duration::from_secs(1) {
        cli.reverse_tunnel_connection_retry_max_backoff = file.reverse_tunnel_connection_retry_max_backoff;
    }
    if cli.tls_sni_override.is_none() {
        cli.tls_sni_override = file.tls_sni_override;
    }
    if !cli.tls_sni_disable {
        cli.tls_sni_disable = file.tls_sni_disable;
    }
    if !cli.tls_ech_enable {
        cli.tls_ech_enable = file.tls_ech_enable;
    }
    if !cli.tls_verify_certificate {
        cli.tls_verify_certificate = file.tls_verify_certificate;
    }
    if cli.http_proxy.is_none() {
        cli.http_proxy = file.http_proxy;
    }
    if cli.http_proxy_login.is_none() {
        cli.http_proxy_login = file.http_proxy_login;
    }
    if cli.http_proxy_password.is_none() {
        cli.http_proxy_password = file.http_proxy_password;
    }
    if cli.http_upgrade_path_prefix == wstunnel::config::DEFAULT_CLIENT_UPGRADE_PATH_PREFIX {
        cli.http_upgrade_path_prefix = file.http_upgrade_path_prefix;
    }
    if cli.http_upgrade_credentials.is_none() {
        cli.http_upgrade_credentials = file.http_upgrade_credentials;
    }
    if cli.websocket_ping_frequency == Some(std::time::Duration::from_secs(30)) {
        cli.websocket_ping_frequency = file.websocket_ping_frequency;
    }
    if !cli.websocket_mask_frame {
        cli.websocket_mask_frame = file.websocket_mask_frame;
    }
    if cli.http_headers.is_empty() {
        cli.http_headers = file.http_headers;
    }
    if cli.http_headers_file.is_none() {
        cli.http_headers_file = file.http_headers_file;
    }
    if cli.remote_addr.as_str() == "ws://127.0.0.1:8080/" {
        cli.remote_addr = file.remote_addr;
    }
    if cli.tls_certificate.is_none() {
        cli.tls_certificate = file.tls_certificate;
    }
    if cli.tls_private_key.is_none() {
        cli.tls_private_key = file.tls_private_key;
    }
    if cli.dns_resolver.is_empty() {
        cli.dns_resolver = file.dns_resolver;
    }
    if !cli.dns_resolver_prefer_ipv4 {
        cli.dns_resolver_prefer_ipv4 = file.dns_resolver_prefer_ipv4;
    }
    
    cli
}

fn merge_server_config(mut cli: Server, file_config: Option<Server>) -> Server {
    let Some(file) = file_config else {
        return cli;
    };
    
    // Merge config: CLI args override config file
    if cli.remote_addr.as_str() == "ws://0.0.0.0:8080/" {
        cli.remote_addr = file.remote_addr;
    }
    if cli.socket_so_mark.is_none() {
        cli.socket_so_mark = file.socket_so_mark;
    }
    if cli.websocket_ping_frequency == Some(std::time::Duration::from_secs(30)) {
        cli.websocket_ping_frequency = file.websocket_ping_frequency;
    }
    if !cli.websocket_mask_frame {
        cli.websocket_mask_frame = file.websocket_mask_frame;
    }
    if cli.dns_resolver.is_empty() {
        cli.dns_resolver = file.dns_resolver;
    }
    if !cli.dns_resolver_prefer_ipv4 {
        cli.dns_resolver_prefer_ipv4 = file.dns_resolver_prefer_ipv4;
    }
    if cli.restrict_to.is_none() {
        cli.restrict_to = file.restrict_to;
    }
    if cli.restrict_http_upgrade_path_prefix.is_none() {
        cli.restrict_http_upgrade_path_prefix = file.restrict_http_upgrade_path_prefix;
    }
    if cli.restrict_config.is_none() {
        cli.restrict_config = file.restrict_config;
    }
    if cli.tls_certificate.is_none() {
        cli.tls_certificate = file.tls_certificate;
    }
    if cli.tls_private_key.is_none() {
        cli.tls_private_key = file.tls_private_key;
    }
    if cli.tls_client_ca_certs.is_none() {
        cli.tls_client_ca_certs = file.tls_client_ca_certs;
    }
    if cli.http_proxy.is_none() {
        cli.http_proxy = file.http_proxy;
    }
    if cli.http_proxy_login.is_none() {
        cli.http_proxy_login = file.http_proxy_login;
    }
    if cli.http_proxy_password.is_none() {
        cli.http_proxy_password = file.http_proxy_password;
    }
    if cli.remote_to_local_server_idle_timeout == std::time::Duration::from_secs(180) {
        cli.remote_to_local_server_idle_timeout = file.remote_to_local_server_idle_timeout;
    }
    
    cli
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut args = Wstunnel::parse();

    // Load config file if provided
    let config_file = if let Some(config_path) = &args.config {
        match load_config_file(config_path) {
            Ok(config) => Some(config),
            Err(e) => {
                eprintln!("Warning: Failed to load config file '{}': {}", config_path.display(), e);
                None
            }
        }
    } else {
        None
    };

    // Merge config file with CLI args
    if let Some(config) = config_file {
        match &mut args.commands {
            Commands::Client(client) => {
                **client = merge_client_config((**client).clone(), config.client);
            }
            Commands::Server(server) => {
                **server = merge_server_config((**server).clone(), config.server);
            }
        }
    }

    // Setup logging
    let mut env_filter = EnvFilter::builder().parse(&args.log_lvl).expect("Invalid log level");
    if !(args.log_lvl.contains("h2::") || args.log_lvl.contains("h2=")) {
        env_filter = env_filter.add_directive(Directive::from_str("h2::codec=off").expect("Invalid log directive"));
    }
    let logger = tracing_subscriber::fmt()
        .with_ansi(args.no_color.is_none())
        .with_env_filter(env_filter);

    // stdio tunnel capture stdio, so need to log into stderr
    if let Commands::Client(args) = &args.commands {
        if args
            .local_to_remote
            .iter()
            .filter(|x| matches!(x.local_protocol, LocalProtocol::Stdio { .. }))
            .count()
            > 0
        {
            logger.with_writer(io::stderr).init();
        } else {
            logger.init()
        }
    } else {
        logger.init();
    };
    if let Err(err) = fdlimit::raise_fd_limit() {
        warn!("Failed to set soft filelimit to hard file limit: {}", err)
    }

    match args.commands {
        Commands::Client(args) => {
            run_client(*args, DefaultTokioExecutor::default())
                .await
                .unwrap_or_else(|err| {
                    panic!("Cannot start wstunnel client: {err:?}");
                });
        }
        Commands::Server(args) => {
            run_server(*args, DefaultTokioExecutor::default())
                .await
                .unwrap_or_else(|err| {
                    panic!("Cannot start wstunnel server: {err:?}");
                });
        }
    }

    Ok(())
}
