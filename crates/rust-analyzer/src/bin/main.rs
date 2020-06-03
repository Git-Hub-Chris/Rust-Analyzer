//! Driver for rust-analyzer.
//!
//! Based on cli flags, either spawns an LSP server, or runs a batch analysis
mod args;

use itertools::Itertools;
use lsp_server::Connection;
use rust_analyzer::{cli, config::Config, from_json, show_message, Result};
use rustc_hash::FxHashSet;

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
            with_proc_macro,
        } => cli::analysis_stats(
            args.verbosity,
            memory_usage,
            path.as_ref(),
            only.as_ref().map(String::as_ref),
            with_deps,
            randomize,
            load_output_dirs,
            with_proc_macro,
        )?,

        args::Command::Bench { path, what, load_output_dirs, with_proc_macro } => {
            cli::analysis_bench(
                args.verbosity,
                path.as_ref(),
                what,
                load_output_dirs,
                with_proc_macro,
            )?
        }

        args::Command::Diagnostics { path, load_output_dirs, with_proc_macro, all } => {
            cli::diagnostics(path.as_ref(), load_output_dirs, with_proc_macro, all)?
        }

        args::Command::ProcMacro => run_proc_macro_srv()?,
        args::Command::RunServer => run_server()?,
        args::Command::Version => println!("rust-analyzer {}", env!("REV")),
    }
    Ok(())
}

fn setup_logging() -> Result<()> {
    std::env::set_var("RUST_BACKTRACE", "short");
    env_logger::try_init_from_env("RA_LOG")?;
    ra_prof::init();
    Ok(())
}

fn run_proc_macro_srv() -> Result<()> {
    ra_proc_macro_srv::cli::run()?;
    Ok(())
}

fn run_server() -> Result<()> {
    log::info!("lifecycle: server started");

    let (connection, io_threads) = Connection::stdio();

    let (initialize_id, initialize_params) = connection.initialize_start()?;
    let initialize_params =
        from_json::<lsp_types::InitializeParams>("InitializeParams", initialize_params)?;

    let server_capabilities = rust_analyzer::server_capabilities(&initialize_params.capabilities);

    let initialize_result = lsp_types::InitializeResult {
        capabilities: server_capabilities,
        server_info: Some(lsp_types::ServerInfo {
            name: String::from("rust-analyzer"),
            version: Some(String::from(env!("REV"))),
        }),
    };

    let initialize_result = serde_json::to_value(initialize_result).unwrap();

    connection.initialize_finish(initialize_id, initialize_result)?;

    if let Some(client_info) = initialize_params.client_info {
        log::info!("Client '{}' {}", client_info.name, client_info.version.unwrap_or_default());
    }

    let cwd = std::env::current_dir()?;
    let root = initialize_params.root_uri.and_then(|it| it.to_file_path().ok()).unwrap_or(cwd);

    let workspace_roots = initialize_params
        .workspace_folders
        .map(|workspaces| {
            workspaces.into_iter().filter_map(|it| it.uri.to_file_path().ok()).collect::<Vec<_>>()
        })
        .filter(|workspaces| !workspaces.is_empty())
        .unwrap_or_else(|| vec![root]);

    let config = {
        let mut config = Config::default();
        if let Some(value) = &initialize_params.initialization_options {
            config.update(value);
        }
        config.update_caps(&initialize_params.capabilities);

        config
    };

    let workspaces = {
        // FIXME: support dynamic workspace loading.
        let project_roots: FxHashSet<_> = workspace_roots
            .iter()
            .filter_map(|it| ra_project_model::ProjectRoot::discover(it).ok())
            .flatten()
            .collect();

        if project_roots.is_empty() && config.notifications.cargo_toml_not_found {
            show_message(
                lsp_types::MessageType::Error,
                format!(
                    "rust-analyzer failed to discover workspace, no Cargo.toml found, dirs searched: {}",
                    workspace_roots.iter().format_with(", ", |it, f| f(&it.display()))
                ),
                &connection.sender,
            );
        };

        project_roots
            .into_iter()
            .filter_map(|root| {
                ra_project_model::ProjectWorkspace::load(root, &config.cargo, config.with_sysroot)
                    .map_err(|err| {
                        log::error!("failed to load workspace: {:#}", err);
                        show_message(
                            lsp_types::MessageType::Error,
                            format!("rust-analyzer failed to load workspace: {:#}", err),
                            &connection.sender,
                        );
                    })
                    .ok()
            })
            .collect::<Vec<_>>()
    };

    rust_analyzer::main_loop(workspaces, config, connection)?;

    log::info!("shutting down IO...");
    io_threads.join()?;
    log::info!("... IO is down");
    Ok(())
}
