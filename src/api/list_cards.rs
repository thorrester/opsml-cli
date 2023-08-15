use crate::api::types;
use crate::api::utils;
use reqwest;
use serde_json;
use std::collections::HashMap;
use tabled::settings::style::Style;
use tabled::{settings::Alignment, Table};
enum OpsmlRegistries {
    Data,
    Model,
    Run,
    Pipeline,
    Audit,
}

impl OpsmlRegistries {
    fn as_str(&self) -> String {
        match self {
            OpsmlRegistries::Data => "OPSML_DATA_REGISTRY".to_string(),
            OpsmlRegistries::Model => "OPSML_MODEL_REGISTRY".to_string(),
            OpsmlRegistries::Run => "OPSML_RUN_REGISTRY".to_string(),
            OpsmlRegistries::Pipeline => "OPSML_PIPELINE_REGISTRY".to_string(),
            OpsmlRegistries::Audit => "OPSML_AUDIT_REGISTRY".to_string(),
        }
    }
}

fn get_registry(registry: &str) -> String {
    // Determines correct  registry to use

    if registry == "data" {
        return OpsmlRegistries::Data.as_str();
    } else if registry == "model" {
        return OpsmlRegistries::Model.as_str();
    } else if registry == "run" {
        return OpsmlRegistries::Run.as_str();
    } else if registry == "model" {
        return OpsmlRegistries::Pipeline.as_str();
    } else if registry == "audit" {
        return OpsmlRegistries::Audit.as_str();
    };

    panic!("Failed to find registry {}", registry);
}

fn parse_list_response(response: &str) -> String {
    // Parses response and creates a table

    let cards: types::ListCardResponse =
        serde_json::from_str(response).expect("Failed to load response to CardResponse JSON");

    let mut card_table: Vec<types::CardTable> = Vec::new();

    for card in cards.cards.iter() {
        card_table.push(types::CardTable {
            name: card.name.clone(),
            team: card.team.clone(),
            date: card.date.clone(),
            user_email: card.user_email.clone(),
            version: card.version.clone(),
            uid: card.uid.clone(),
        });
    }

    let list_table = Table::new(card_table)
        .with(Alignment::center())
        .with(Style::sharp())
        .to_string();

    return list_table;
}

/// List cards
///     
/// # Arguments
///
/// * `registry` - Registry to list cards from
/// * `name` - Name of card
/// * `team` - Team name
/// * `version` - Card version
/// * `uid` - Card uid
/// * `limit` - Limit number of cards returned
/// * `url` - OpsML url
/// * `tag_name` - Tag name
/// * `tag_value` - Tag value
#[tokio::main]
pub async fn list_cards(
    registry: &str,
    name: Option<&str>,
    team: Option<&str>,
    version: Option<&str>,
    uid: Option<&str>,
    limit: Option<i16>,
    tag_name: Option<Vec<String>>,
    tag_value: Option<Vec<String>>,
    max_date: Option<&str>,
) -> Result<(), reqwest::Error> {
    // set full path and table name

    let mut tags: HashMap<String, String> = HashMap::new();
    let table_name: String = get_registry(&registry);

    if tag_name.is_some() && tag_value.is_some() {
        tags = tag_name
            .unwrap()
            .iter()
            .zip(tag_value.unwrap().iter())
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
    }

    let list_table_request = types::ListTableRequest {
        table_name: table_name,
        name: name.map(|s| s.to_string()),
        team: team.map(|s| s.to_string()),
        version: version.map(|s| s.to_string()),
        limit: limit,
        uid: uid.map(|s| s.to_string()),
        tags: Some(tags),
        max_date: max_date.map(|s| s.to_string()),
    };

    let response =
        utils::make_post_request(&utils::OpsmlPaths::ListCard.as_str(), &list_table_request).await;

    if response.status().is_success() {
        let card_table = parse_list_response(&response.text().await.unwrap());
        println!("{}", card_table);
    } else {
        println!("Failed to list cards");
        response.error_for_status_ref()?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_registry() {
        let v = vec!["data", "model", "run", "audit"];

        for name in &v {
            let regsitry: String = get_registry(name);
            let expected_registry: String = format!("OPSML_{}_REGISTRY", name.to_uppercase());
            assert_eq!(&regsitry, &expected_registry);
        }
    }

    #[test]
    fn test_parse_response() {
        let mut vec = Vec::new();
        let card = types::Card {
            name: "test".to_string(),
            team: "test".to_string(),
            date: "test".to_string(),
            user_email: "fake_email".to_string(),
            version: "1.0.0".to_string(),
            uid: "uid".to_string(),
            tags: HashMap::new(),
        };
        vec.push(card);

        let mock_response = types::ListCardResponse { cards: vec };
        let string_response = serde_json::to_string(&mock_response).unwrap();

        let card_table = parse_list_response(&string_response);
        assert_eq!(
            card_table,
            concat!(
                "┌──────┬──────┬──────┬────────────┬─────────┬─────┐\n",
                "│ name │ team │ date │ user_email │ version │ uid │\n",
                "├──────┼──────┼──────┼────────────┼─────────┼─────┤\n",
                "│ test │ test │ test │ fake_email │  1.0.0  │ uid │\n",
                "└──────┴──────┴──────┴────────────┴─────────┴─────┘",
            )
        );
    }
}
