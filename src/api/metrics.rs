use crate::api::types;
use crate::api::utils;
use reqwest;
use tabled::settings::style::Style;
use tabled::{settings::Alignment, Table};

fn parse_metric_response(response: &str) -> String {
    // Parses response and creates a table

    let metrics: types::ListMetricResponse =
        serde_json::from_str(response).expect("Failed to load response to MetricResponse JSON");

    let mut metric_table: Vec<types::MetricTable> = Vec::new();

    for (_, metric_array) in metrics.metrics.iter() {
        for metric in metric_array.iter() {
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
                metric: metric.name.clone(),
                value: metric.value.clone(),
                step: step,
                timestamp: timestamp,
            });
        }
    }

    let metric_table = Table::new(metric_table)
        .with(Alignment::center())
        .with(Style::sharp())
        .to_string();

    return metric_table;
}

/// List all metrics for a model
///
/// # Arguments
///
/// * `name` - Name of the model
/// * `version` - Version of the model
/// * `uid` - Unique identifier of the model
/// * `url` - URL of the OpsML server
#[tokio::main]
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
        let metric_table = parse_metric_response(&response.text().await?);
        println!("{}", metric_table);
    } else {
        println!("Failed to get metrics for model");
        response.error_for_status_ref()?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_parse_metric_response() {
        let mut vec = Vec::new();
        let metric1 = types::Metric {
            name: "mae".to_string(),
            value: 5.into(),
            step: None,
            timestamp: None,
        };
        vec.push(metric1);

        let metric2 = types::Metric {
            name: "mape".to_string(),
            value: 10.0.into(),
            step: None,
            timestamp: None,
        };
        vec.push(metric2);

        let mut metrics = HashMap::new();
        metrics.insert("test".to_string(), vec);

        let mock_response = types::ListMetricResponse { metrics: metrics };
        let string_response = serde_json::to_string(&mock_response).unwrap();

        let metric_table = parse_metric_response(&string_response);

        assert_eq!(
            metric_table,
            concat!(
                "┌────────┬───────┬──────┬───────────┐\n",
                "│ metric │ value │ step │ timestamp │\n",
                "├────────┼───────┼──────┼───────────┤\n",
                "│  mae   │   5   │ None │   None    │\n",
                "│  mape  │ 10.0  │ None │   None    │\n",
                "└────────┴───────┴──────┴───────────┘",
            )
        )
    }
}
