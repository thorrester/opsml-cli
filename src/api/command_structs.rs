use clap::Args;

#[derive(Args)]
pub struct ListCards {
    /// Name of the registry (data, model, run, etc)
    #[arg(long = "registry")]
    pub registry: String,

    /// Name given to a card
    #[arg(long = "name")]
    pub name: Option<String>,

    /// Team name
    #[arg(long = "team")]
    pub team: Option<String>,

    /// Card version
    #[arg(long = "version")]
    pub version: Option<String>,

    /// Card uid
    #[arg(long = "uid")]
    pub uid: Option<String>,

    /// Card limit
    #[arg(long = "limit")]
    pub limit: Option<i16>,

    /// Tag name
    #[arg(long = "tag_name", use_value_delimiter = true, value_delimiter = ',')]
    pub tag_name: Option<Vec<String>>,

    /// Tag values
    #[arg(long = "tag_value", use_value_delimiter = true, value_delimiter = ',')]
    pub tag_value: Option<Vec<String>>,

    /// max date
    #[arg(long = "max_date")]
    pub max_date: Option<String>,
}

#[derive(Args)]
pub struct ModelMetadataArgs {
    /// Name given to card
    #[arg(long = "name")]
    pub name: Option<String>,

    /// Card version
    #[arg(long = "version")]
    pub version: Option<String>,

    /// Card uid
    #[arg(long = "uid")]
    pub uid: Option<String>,

    /// Write directory
    #[arg(long = "write-dir", default_value = ".models")]
    pub write_dir: String,
}

#[derive(Args)]
pub struct DownloadModelArgs {
    /// Name given to card
    #[arg(long = "name")]
    pub name: Option<String>,

    /// Card version
    #[arg(long = "version")]
    pub version: Option<String>,

    /// Card uid
    #[arg(long = "uid")]
    pub uid: Option<String>,

    /// Write directory
    #[arg(long = "write-dir", default_value = ".models")]
    pub write_dir: String,

    /// Boolean indicating whether to download onnx or trained model
    #[arg(long = "no-onnx", default_value = "false")]
    pub no_onnx: bool,

    /// Boolean indicating whether to download onnx or trained model
    #[arg(long = "onnx", default_value = "true")]
    pub onnx: bool,
}

#[derive(Args)]
pub struct ModelMetricArgs {
    /// Name given to card
    #[arg(long = "name")]
    pub name: Option<String>,

    /// Card version
    #[arg(long = "version")]
    pub version: Option<String>,

    /// Card uid
    #[arg(long = "uid")]
    pub uid: Option<String>,
}
