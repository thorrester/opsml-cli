use crate::api::types;
use crate::api::utils;
use reqwest;
use tabled::settings::style::Style;
use tabled::{settings::Alignment, Table};

fn parse_metric_response(response: &str) {
    // Parses response and creates a table

    let metrics: types::ListMetricResponse = serde_json::from_str(response).unwrap();

    let mut metric_table: Vec<types::MetricTable> = Vec::new();

    for metric in metrics.metrics.iter() {
        let step = if metric.step.is_some() {
            metric.step.as_ref().unwrap().to_string()
        } else {
            "None".to_string()
        };

        let timestamp = if metric.timestamp.is_some() {
            metric.timestamp.as_ref().unwrap().to_string()
        } else {
            "None".to_string()
        };

        metric_table.push(types::MetricTable {
            name: metric.name.clone(),
            value: metric.value.clone(),
            step: step,
            timestamp: timestamp,
        });
    }

    let metric_table = Table::new(metric_table)
        .with(Alignment::center())
        .with(Style::sharp())
        .to_string();

    println!("{}", metric_table);
}

pub async fn get_model_metrics(
    name: Option<&str>,
    version: Option<&str>,
    uid: Option<&str>,
    url: &str,
) -> Result<(), reqwest::Error> {
    let full_uri_path: String = format!("{}/opsml/models/metrics", url);

    let model_metric_request = types::CardRequest {
        name: name.map(|s| s.to_string()),
        version: version.map(|s| s.to_string()),
        uid: uid.map(|s| s.to_string()),
    };

    let response = utils::make_post_request(&full_uri_path, &model_metric_request).await;

    if response.status().is_success() {
        parse_metric_response(&response.text().await?);
    } else {
        println!("Failed to list cards");
        response.error_for_status_ref()?;
    }

    Ok(())
}
