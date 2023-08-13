pub fn check_args(
    name: &Option<String>,
    team: &Option<String>,
    version: &Option<String>,
    uid: &Option<String>,
) -> Result<(), &'static str> {
    let common_args = vec![name, team, version];
    let has_common = common_args.iter().all(|i| i.is_none());

    let has_uid = uid.is_none();

    if has_common != has_uid {
        Ok(())
    } else {
        Err("Either name/team/version or uid must be specified")
    }
}

pub fn remove_suffix<'a>(s: &str, suffix: &str) -> String {
    match s.strip_suffix(suffix) {
        Some(s) => s.to_string(),
        None => s.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_suffix() {
        let test_uri_with_slash = "http://localhost:8080/";
        let test_uri_without_slash = "http://localhost:8080";
        let processed_with_slash_uri = remove_suffix(&test_uri_with_slash, "/");
        let processed_without_slash_uri = remove_suffix(&test_uri_without_slash, "/");
        assert_eq!(processed_with_slash_uri, "http://localhost:8080");
        assert_eq!(processed_without_slash_uri, test_uri_without_slash);
    }
}
