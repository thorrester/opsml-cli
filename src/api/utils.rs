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
