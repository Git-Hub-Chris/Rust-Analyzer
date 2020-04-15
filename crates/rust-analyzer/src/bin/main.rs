//! Driver for rust-analyzer.
//!
//! Based on cli flags, either spawns an LSP server, or runs a batch analysis
mod args;

use lsp_server::Connection;
use rust_analyzer::{cli, config::Config, from_json, Result};

use crate::args::HelpPrinted;

fn main() -> Result<()> {
    setup_logging()?;
    let args = match args::Args::parse()? {
        Ok(it) => it,
        Err(HelpPrinted) => return Ok(()),
    };
    match args.command {
        args::Command::Parse { no_dump } => cli::parse(no_dump)?,
        args::Command::Symbols => cli::symbols()?,
        args::Command::Highlight { rainbow } => cli::highlight(rainbow)?,
        args::Command::Stats {
            randomize,
            memory_usage,
            only,
            with_deps,
            path,
            load_output_dirs,
        } => cli::analysis_stats(
            args.verbosity,
            memory_usage,
            path.as_ref(),
            only.as_ref().map(String::as_ref),
            with_deps,
            randomize,
            load_output_dirs,
        )?,

        args::Command::Bench { path, what, load_output_dirs } => {
            cli::analysis_bench(args.verbosity, path.as_ref(), what, load_output_dirs)?
        }

        args::Command::Diagnostics { path, load_output_dirs, all } => {
            cli::diagnostics(path.as_ref(), load_output_dirs, all)?
        }

        args::Command::RunServer => run_server(args.roots)?,
        args::Command::Version => println!("rust-analyzer {}", env!("REV")),
    }
    Ok(())
}

fn setup_logging() -> Result<()> {
    std::env::set_var("RUST_BACKTRACE", "short");
    env_logger::try_init()?;
    ra_prof::init();
    Ok(())
}

fn run_server(roots: Option<Vec<String>>) -> Result<()> {
    log::info!("lifecycle: server started");

    let (connection, io_threads) = Connection::stdio();
    let server_capabilities = serde_json::to_value(rust_analyzer::server_capabilities()).unwrap();

    let initialize_params = connection.initialize(server_capabilities)?;
    let initialize_params =
        from_json::<lsp_types::InitializeParams>("InitializeParams", initialize_params)?;

    if let Some(client_info) = initialize_params.client_info {
        log::info!("Client '{}' {}", client_info.name, client_info.version.unwrap_or_default());
    }

    let cwd = std::env::current_dir()?;
    let root = initialize_params.root_uri.and_then(|it| it.to_file_path().ok()).unwrap_or(cwd);

    let workspace_roots = if let Some(roots) = roots {
        roots.into_iter().map(Into::into).collect::<Vec<_>>()
    } else {
        initialize_params
            .workspace_folders
            .map(|workspaces| {
                workspaces
                    .into_iter()
                    .filter_map(|it| it.uri.to_file_path().ok())
                    .collect::<Vec<_>>()
            })
            .filter(|workspaces| !workspaces.is_empty())
            .unwrap_or_else(|| vec![root])
    };

    let config = {
        let mut config = Config::default();
        if let Some(value) = &initialize_params.initialization_options {
            config.update(value);
        }
        if let Some(caps) = &initialize_params.capabilities.text_document {
            config.update_caps(caps);
        }
        config
    };

    rust_analyzer::main_loop(workspace_roots, config, connection)?;

    log::info!("shutting down IO...");
    io_threads.join()?;
    log::info!("... IO is down");
    Ok(())
}
