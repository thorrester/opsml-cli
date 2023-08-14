use api::download_file::download_model;
use api::download_file::download_model_metadata;
use api::list_cards::list_cards;
use api::utils::remove_suffix;
mod api;

use clap::command;
use clap::Args;
use clap::Parser;
use clap::Subcommand;
use lazy_static::lazy_static;
use std::env;

lazy_static! {
    static ref OPSML_TRACKING_URI: String = match env::var("OPSML_TRACKING_URI") {
        Ok(val) =>
            if val.ends_with("/") {
                remove_suffix(&val, "/")
            } else {
                val
            },
        Err(_e) => panic!("No tracking uri set"),
    };
}

#[derive(Parser)]
#[command(about = "CLI tool for Interacting with Opsml server")]

struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Lists registered cards
    ListCards(ListCards),
    DownloadModelMetadata(ModelMetadataArgs),
    DownloadModel(DownloadModelArgs),
}

#[derive(Args)]
struct ListCards {
    /// Name of the registry (data, model, run, etc)
    #[arg(long = "registry")]
    registry: String,

    /// Name given to card
    #[arg(long = "name")]
    name: Option<String>,

    /// Team name
    #[arg(long = "team")]
    team: Option<String>,

    /// Card version
    #[arg(long = "version")]
    version: Option<String>,

    /// Card uid
    #[arg(long = "uid")]
    uid: Option<String>,

    /// Card limit
    #[arg(long = "limit")]
    limit: Option<i16>,

    /// Tag name
    #[arg(long = "tag_name", use_value_delimiter = true, value_delimiter = ',')]
    tag_name: Option<Vec<String>>,

    /// Tag values
    #[arg(long = "tag_value", use_value_delimiter = true, value_delimiter = ',')]
    tag_value: Option<Vec<String>>,

    /// max date
    #[arg(long = "max_date")]
    max_date: Option<String>,
}

#[derive(Args)]
struct ModelMetadataArgs {
    /// Name given to card
    #[arg(long = "name")]
    name: Option<String>,

    /// Team name
    #[arg(long = "team")]
    team: Option<String>,

    /// Card version
    #[arg(long = "version")]
    version: Option<String>,

    /// Card uid
    #[arg(long = "uid")]
    uid: Option<String>,

    /// Write directory
    #[arg(long = "write-dir", default_value = ".models")]
    write_dir: String,
}

#[derive(Args)]
struct DownloadModelArgs {
    /// Name given to card
    #[arg(long = "name")]
    name: Option<String>,

    /// Team name
    #[arg(long = "team")]
    team: Option<String>,

    /// Card version
    #[arg(long = "version")]
    version: Option<String>,

    /// Card uid
    #[arg(long = "uid")]
    uid: Option<String>,

    /// Write directory
    #[arg(long = "write-dir", default_value = ".models")]
    write_dir: String,

    /// Boolean indicating whether to download onnx or trained model
    #[arg(long = "no-onnx", default_value = "false")]
    no_onnx: bool,

    /// Boolean indicating whether to download onnx or trained model
    #[arg(long = "onnx", default_value = "true")]
    onnx: bool,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        // subcommand for list cards
        Some(Commands::ListCards(args)) => {
            let response = list_cards(
                args.registry.as_str(),
                args.name.as_deref(),
                args.team.as_deref(),
                args.version.as_deref(),
                args.uid.as_deref(),
                args.limit.clone(),
                &*OPSML_TRACKING_URI,
                args.tag_name.clone(),
                args.tag_value.clone(),
                args.max_date.as_deref(),
            );
            match response {
                Ok(response) => response,
                Err(error) => panic!("Problem encountered: {:?}", error),
            };
        }

        // subcommand for downloading model metadata
        Some(Commands::DownloadModelMetadata(args)) => {
            download_model_metadata(
                args.name.clone(),
                args.team.clone(),
                args.version.clone(),
                args.uid.clone(),
                &*OPSML_TRACKING_URI,
                &args.write_dir.clone(),
            );
        }
        // subcommand for downloading a model
        Some(Commands::DownloadModel(args)) => {
            download_model(
                args.name.clone(),
                args.team.clone(),
                args.version.clone(),
                args.uid.clone(),
                &*OPSML_TRACKING_URI,
                &args.write_dir.clone(),
                args.no_onnx.clone(),
                args.onnx.clone(),
            );
        }
        None => {}
    }
}
