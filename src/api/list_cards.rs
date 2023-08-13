use crate::api::types;
use reqwest;
use serde_json;
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
            OpsmlRegistries::Model => "OPSML_Model_REGISTRY".to_string(),
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

fn parse_response(response: &str) {
    // Parses response and creates a table

    let cards: types::ListCardResponse = serde_json::from_str(response).unwrap();

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

    println!("{}", list_table);
}

pub fn list_cards(
    registry: &str,
    name: Option<&str>,
    team: Option<&str>,
    version: Option<&str>,
    uid: Option<&str>,
    limit: Option<i16>,
    url: &str,
) -> Result<(), reqwest::Error> {
    // set full path and table name

    let full_uri_path: String = format!("{}/opsml/cards/list", url);
    let table_name: String = get_registry(&registry);

    let list_table_request = types::ListTableRequest {
        table_name: table_name,
        name: name.map(|s| s.to_string()),
        team: team.map(|s| s.to_string()),
        version: version.map(|s| s.to_string()),
        limit: limit,
        uid: uid.map(|s| s.to_string()),
    };

    let client = reqwest::blocking::Client::new();
    let response = client
        .post(full_uri_path)
        .json(&list_table_request)
        .send()?;

    if response.status().is_success() {
        parse_response(&response.text()?);
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
        let v = vec!["data", "model", "run"];

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
        };
        vec.push(card);

        let mock_response = types::ListCardResponse { cards: vec };
        let string_response = serde_json::to_string(&mock_response).unwrap();

        parse_response(&string_response);
    }
}
