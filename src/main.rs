use clap::{Parser, Subcommand};
use color_eyre::Result;
use dapplication::interactors::terminal_interactor::TerminalInteractor;
use ddomain::repositories::ticket_repository::TicketRepository;
use dinfrastructure::ticket_repository_impl::TicketRepositoryImpl;
use dpresentation::{
    controllers::terminal_controller::TerminalController,
    presenters::ratatui_presenter::RatatuiPresenter,
};
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

            let repository: Box<dyn TicketRepository> =
                Box::new(TicketRepositoryImpl::new(file_path.clone()));

            // ファイルが存在しない場合、リポジトリ側でファイルを生成
            repository.ensure_file_exists_with_template()?;

            println!("新しいファイルが生成されました: {}", file_path);
        }
        Commands::Run { file_name } => {
            let file_path = if Path::new(&file_name).extension().is_some() {
                file_name
            } else {
                format!("{}.toml", file_name)
            };

            let repository = TicketRepositoryImpl::new(file_path.clone());
            let presenter = RatatuiPresenter::new();

            // ファイルが存在しない場合、リポジトリ側でファイルを生成
            repository.ensure_file_exists_with_template()?;

            // TerminalInteractorを使ってTerminalControllerを生成
            let terminal_interactor = TerminalInteractor::new(repository, presenter)?;

            // エラー処理が成功した場合にのみTerminalControllerを作成
            let terminal_controller = TerminalController::new(terminal_interactor);

            // ターミナルコントローラの実行
            terminal_controller.run(ratatui::init())?;

            ratatui::restore();
        }
    }

    Ok(())
}
