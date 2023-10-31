// Copyright © ArkBig

pub fn notify(
    args: &crate::cli_args::SlackArgs,
    notify_url: &str,
    update: &crate::redmine::UpdateInfo,
) -> anyhow::Result<()> {
    let template_path = &args.template_path;

    let msg = convert_to_post_msg(template_path, update)?;
    crate::log::debug(&msg).category("slack");

    ureq::post(notify_url)
        .set("Content-Type", "application/json")
        .send_json(ureq::json!({ "text": msg }))?;

    Ok(())
}

fn convert_to_post_msg(
    template_path: &Option<String>,
    update: &crate::redmine::UpdateInfo,
) -> anyhow::Result<String> {
    // Load template
    let template = if let Some(path) = template_path {
        std::fs::read_to_string(path).unwrap_or_else(|e| {
            let msg = format!(
                "Could not read template from file [{path}] with {e}",
                path = path,
                e = e
            );
            crate::log::error(&msg).category("slack");
            std::process::exit(1);
        })
    } else {
        include_str!("../resources/slack-notification.template").to_string()
    };

    // Add referenced key of new_issue
    let mut update = update.clone();
    let new_referenced_re = regex::Regex::new(r"\Wnew_issue.(?<key>[a-zA-Z_-]+)").unwrap();
    for key in new_referenced_re
        .captures_iter(&template)
        .map(|c| c["key"].to_string())
    {
        if let Some(map) = update.new_issue.as_object_mut() {
            if !map.contains_key(&key) {
                map.insert(key, serde_json::Value::Null);
            }
        }
    }
    // Add referenced key of old_items
    let old_referenced_re = regex::Regex::new(r"\Wold_items.(?<key>[a-zA-Z_-]+)").unwrap();
    for key in old_referenced_re
        .captures_iter(&template)
        .map(|c| c["key"].to_string())
    {
        if let Some(map) = update.old_items.as_object_mut() {
            if !map.contains_key(&key) {
                map.insert(key, serde_json::Value::Null);
            }
        }
    }

    // Render template
    let mut tt = tinytemplate::TinyTemplate::new();
    tt.set_default_formatter(&tinytemplate::format_unescaped);
    tt.add_template("slack-notification", &template)?;
    let msg = tt.render("slack-notification", &update)?;

    Ok(msg)
}
