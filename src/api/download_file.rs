use crate::api::types;
use crate::api::utils;
use futures_util::StreamExt;
use reqwest::{self, Response};
use serde::Serialize;
use serde_json;
use std::{format, fs, path::Path};
use tokio;

const MODEL_METADATA_FILE: &str = "metadata.json";

/// async post request for metadata
async fn make_post_request<T: Serialize>(url: &str, payload: &T) -> Response {
    let parsed_url = reqwest::Url::parse(url).unwrap();
    let client = reqwest::Client::new();

    return client.post(parsed_url).json(payload).send().await.unwrap();
}

/// Parses stream response
///
/// * `response` - Response object
async fn load_stream_response(response: Response) -> String {
    let mut response_stream = response.bytes_stream();
    let mut stream_buffer = String::new();
    while let Some(item) = response_stream.next().await {
        let chunk = item.unwrap();
        let string_chunk = std::str::from_utf8(&chunk).unwrap();

        stream_buffer.push_str(string_chunk);
    }
    return stream_buffer;
}

async fn download_stream_to_file(response: Response, filename: &Path) {
    let mut response_stream = response.bytes_stream();
    let mut file = tokio::fs::File::create(filename).await.unwrap();

    while let Some(item) = response_stream.next().await {
        let chunk = item.unwrap();
        tokio::io::copy(&mut chunk.as_ref(), &mut file)
            .await
            .unwrap();
    }
}

/// Create parent directories associated with path
///
/// * `path` - path to create
fn create_dir_path(path: &str) {
    let path = std::path::Path::new(path);
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();
}

/// Saves struct to json
///
/// * `metadata` - Model metadata struct
fn save_metadata_to_json(metadata: &types::ModelMetadata, path: &str) -> std::io::Result<()> {
    let json_string = serde_json::to_string(metadata).unwrap();
    fs::File::create(path).expect("Unable to create metadata file");
    fs::write(path, &json_string).expect("Unable to write file");

    Ok(())
}

/// Downloads a model file
async fn download_model_file(url: &str, model_uri: &str, local_save_path: &str) {
    let payload = types::ModelDownloadRequest {
        read_path: model_uri.to_string(),
    };

    let response = make_post_request(&url, &payload).await;
    let filepath = Path::new(local_save_path);

    download_stream_to_file(response, filepath).await;
}

/// Main function for downloading model metadata
async fn get_model_metadata(
    name: Option<String>,
    version: Option<String>,
    uid: Option<String>,
    url: &str,
    write_dir: &str,
) -> types::ModelMetadata {
    let full_uri_path: String = format!("{}/opsml/models/metadata", url);
    let save_path: String = format!("{}/{}", write_dir, MODEL_METADATA_FILE);

    let model_metadata_request = types::DownloadMetadataRequest {
        name: name,
        version: version,
        uid: uid,
    };

    let response = make_post_request(&full_uri_path, &model_metadata_request).await;

    let loaded_response = load_stream_response(response).await;
    let model_metadata: types::ModelMetadata = serde_json::from_str(&loaded_response).unwrap();

    // create save path for metadata
    create_dir_path(&save_path);
    save_metadata_to_json(&model_metadata, &save_path).unwrap();

    return model_metadata;
}

/// Sets model uri (onnx or trained model) depending on boolean
fn get_model_uri(onnx: bool, model_metadata: &types::ModelMetadata) -> (String, String) {
    let uri = if onnx {
        if model_metadata.onnx_uri.is_none() {
            panic!("No onnx model uri found");
        } else {
            model_metadata.onnx_uri.clone().unwrap()
        }
    } else {
        model_metadata.model_uri.clone()
    };

    let filepath = std::path::Path::new(&uri);
    let filename = filepath.file_name().unwrap().to_str().unwrap().to_string();

    (filename, uri)
}

/// Downloads model metadata
///
/// * `name` - Name of model
/// * `team` - Team associated with model
/// * `version` - Version of model
/// * `uid` - uid of model
/// * `url` - url of opsml server
#[tokio::main]
pub async fn download_model_metadata(
    name: Option<String>,
    version: Option<String>,
    uid: Option<String>,
    url: &str,
    write_dir: &str,
) -> types::ModelMetadata {
    // check args first
    utils::check_args(&name, &version, &uid).unwrap();
    return get_model_metadata(name, version, uid, url, write_dir).await;
}

/// Downloads model file
///
/// * `name` - Name of model
/// * `team` - Team associated with model
/// * `version` - Version of model
/// * `uid` - uid of model
/// * `url` - url of opsml server
/// * `write_dir` - directory to write to
/// * `no_onnx` - Flag to not download onnx model
/// * `onnx` - Flag to download onnx model
#[tokio::main]
pub async fn download_model(
    name: Option<String>,
    version: Option<String>,
    uid: Option<String>,
    url: &str,
    write_dir: &str,
    no_onnx: bool,
    onnx: bool,
) {
    // check args first
    utils::check_args(&name, &version, &uid).unwrap();

    // If no onnx is set to true, we need to cancel out onnx: true
    // Clap does not currently support command line negation flags

    let download_onnx = if onnx && no_onnx { false } else { true };

    let model_metadata = get_model_metadata(name, version, uid, url, write_dir).await;

    let download_url: String = format!("{}/opsml/files/download", url);

    let (filename, model_uri) = get_model_uri(download_onnx, &model_metadata);

    println!("Downloading model: {}, {}", filename, model_uri);

    let local_save_path = format!("{}/{}", write_dir, filename);

    // Create all parent dirs if not exist
    create_dir_path(&local_save_path);

    // Download model
    download_model_file(&download_url, &model_uri, &local_save_path).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_json_diff::assert_json_eq;
    use std::fs;
    use tokio;

    #[tokio::test]
    async fn test_parse_response() {
        // read mock response object
        let path = "./src/api/test_utils/metadata_non_onnx.json";
        let data = fs::read_to_string(path).expect("Unable to read file");
        let mock_metadata: types::ModelMetadata = serde_json::from_str(&data).unwrap();

        let mut server = mockito::Server::new();
        let url = server.url();

        // Create a mock server
        let mock = server
            .mock("GET", "/fake")
            .with_status(201)
            .with_body(data)
            .create();

        // create client and parse the server response
        let client = reqwest::Client::new();
        let full_path: String = format!("{}/fake", &url);
        let response = client.get(&full_path).send().await.unwrap();
        let loaded_response = load_stream_response(response).await;
        let model_metadata: types::ModelMetadata = serde_json::from_str(&loaded_response).unwrap();

        // assert structs are the same
        assert_json_eq!(mock_metadata, model_metadata);

        mock.assert()
    }

    #[test]
    fn test_save_json() {
        // read mock response object
        let path = "./src/api/test_utils/metadata_onnx.json";
        let data = fs::read_to_string(path).expect("Unable to read file");
        let mock_metadata_orig: types::ModelMetadata = serde_json::from_str(&data).unwrap();
        let new_path = "./src/api/test_utils/new_mock_response.json";

        save_metadata_to_json(&mock_metadata_orig, new_path).unwrap();

        let new_data = fs::read_to_string(new_path).expect("Unable to read file");

        // confirm new json can be loaded in
        let mock_metadata: types::ModelMetadata = serde_json::from_str(&new_data).unwrap();
        assert_json_eq!(mock_metadata, mock_metadata_orig);

        // clean up
        fs::remove_file(new_path).unwrap();
    }

    #[test]
    fn test_model_metadata_loading() {
        // read model metadata
        let path = "./src/api/test_utils/metadata_onnx.json";
        let data = fs::read_to_string(path).expect("Unable to read file");
        let mock_metadata: types::ModelMetadata = serde_json::from_str(&data).unwrap();
        assert!(mock_metadata.onnx_uri.is_some());

        // read model metadata without onnx
        let path = "./src/api/test_utils/metadata_non_onnx.json";
        let data = fs::read_to_string(path).expect("Unable to read file");
        let mock_metadata_non_onnx: types::ModelMetadata = serde_json::from_str(&data).unwrap();
        assert!(mock_metadata_non_onnx.onnx_uri.is_none());
    }

    #[tokio::test]
    async fn test_download_stream_to_file() {
        let mut server = mockito::Server::new();
        let url = server.url();
        let path = "./src/api/test_utils/metadata_onnx.json";
        let new_path = "./src/api/test_utils/new_mock_response.json";

        // Create a mock server
        let mock = server
            .mock("GET", "/fake")
            .with_status(201)
            .with_body_from_file(path)
            .create();

        let client = reqwest::Client::new();
        let full_path: String = format!("{}/fake", &url);
        let response = client.get(&full_path).send().await.unwrap();

        download_stream_to_file(response, Path::new(new_path)).await;
        mock.assert();
        fs::remove_file(new_path).unwrap();
    }

    #[tokio::test]
    async fn test_make_post_request() {
        let mut server = mockito::Server::new();
        let url = server.url();
        let path = "./src/api/test_utils/metadata_onnx.json";
        let payload = types::ModelDownloadRequest {
            read_path: "mock_path".to_string(),
        };

        // Create a mock server
        let mock = server
            .mock("POST", "/fake")
            .with_status(201)
            .with_body_from_file(path)
            .create();

        let full_path: String = format!("{}/fake", &url);
        let response = make_post_request(&full_path, &payload).await;

        assert_eq!(response.status(), 201);
        mock.assert();
    }
}
