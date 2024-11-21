mod app;
mod app_errors;

use crate::app::App;
use crate::app::Ticket;
use clap::{Parser, Subcommand};
use color_eyre::Result;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(name = "Digger")]
#[command(version = "1.0")]
#[command(about = "Manage tickets using a TOML file", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    New { file_name: String },
    Run { file_name: String },
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::try_parse()?;

    match cli.command {
        Commands::New { file_name } => {
            let file_path = if Path::new(&file_name).extension().is_some() {
                file_name
            } else {
                format!("{}.toml", file_name)
            };

            ensure_file_exists_with_template(&file_path)?;
            println!("新しいファイルが生成されました: {}", file_path);
        }
        Commands::Run { file_name } => {
            let file_path = if Path::new(&file_name).extension().is_some() {
                file_name
            } else {
                format!("{}.toml", file_name)
            };

            ensure_file_exists_with_template(&file_path)?;
            let tickets_data = deserial_toml_file::<app::Tickets>(&file_path)?;

            let terminal = ratatui::init();
            App::new(tickets_data.tickets).run(terminal)?;
            ratatui::restore();
        }
    }

    Ok(())
}

fn deserial_toml_file<T>(path: &str) -> Result<T, crate::app_errors::AppError>
where
    T: for<'a> Deserialize<'a>,
{
    let file_str = std::fs::read_to_string(path).map_err(crate::app_errors::AppError::FileRead)?;
    if file_str.trim().is_empty() {
        Err(crate::app_errors::AppError::EmptyFile)
    } else {
        toml::from_str(&file_str).map_err(crate::app_errors::AppError::TomlParse)
    }
}

fn ensure_file_exists_with_template(file_path: &str) -> Result<()> {
    let path = Path::new(file_path);

    if !path.exists() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        // デフォルトのチケットリストを生成
        let default_ticket = Ticket::default();

        let tickets = app::Tickets {
            tickets: vec![default_ticket],
        };

        // TOML のシリアライズ
        let template = toml::to_string_pretty(&tickets)?;
        fs::write(file_path, template)?;
    }
    Ok(())
}
