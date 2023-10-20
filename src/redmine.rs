// Copyright © ArkBig

use std::collections::HashMap;

#[derive(serde::Serialize, Clone)]
pub struct UpdateContent {
    pub author: String,
    pub content: String,
}

#[derive(serde::Serialize, Clone)]
pub struct UpdateInfo {
    pub url: String,
    pub local_updated_time: chrono::DateTime<chrono::Local>,
    pub new_issue: serde_json::Value,
    pub old_items: serde_json::Value,
    pub update_contents: Vec<UpdateContent>,
}

pub fn load_prev_data(args: &crate::cli_args::RedmineArgs) -> anyhow::Result<RedmineData> {
    let prev_data_path = &args.prev_redmine_data;
    let prev_data = if std::path::Path::new(prev_data_path).exists() {
        let prev_data = std::fs::read_to_string(prev_data_path)?;
        if prev_data.is_empty() {
            RedmineData {
                prev_date: chrono::DateTime::<chrono::Utc>::MIN_UTC,
                issues: HashMap::new(),
            }
        } else {
            let prev_data: RedmineData = serde_json::from_str(&prev_data)?;
            prev_data
        }
    } else {
        RedmineData {
            prev_date: chrono::DateTime::<chrono::Utc>::MIN_UTC,
            issues: HashMap::new(),
        }
    };
    Ok(prev_data)
}

pub fn refresh_updated_issues(
    args: &crate::cli_args::RedmineArgs,
    prev_data: &mut RedmineData,
) -> anyhow::Result<Vec<UpdateInfo>> {
    let url = &args.subscribe_url;
    let atom_key = &args.redmine_atom_key;
    let api_key = &args.redmine_api_key;
    let max_content_length = args.max_content_length;
    let filter = &args.filter;

    // List the activities grouped by id.
    let activity_map = get_activities(
        prev_data.prev_date,
        url,
        atom_key,
        filter,
        max_content_length,
    )?;
    if activity_map.is_empty() {
        return Ok(vec![]);
    }

    // Get the issue information from the activity.
    let updated_ids = activity_map.keys().map(|id| *id).collect::<Vec<_>>();
    let new_issues = get_issues(url, api_key, &updated_ids)?;

    // Create a list of updates.
    let mut updates = Vec::new();
    for (id, new_issue) in &new_issues {
        let old_issue = prev_data.issues.get(&id);
        let old_items = get_updated_items(&old_issue, &new_issue)?;
        let local_updated_time =
            chrono::DateTime::parse_from_rfc3339(new_issue["updated_on"].as_str().unwrap())?
                .with_timezone(&chrono::Local);
        updates.push(UpdateInfo {
            url: format!("{}/issues/{}", url, &new_issue["id"].as_u64().unwrap()),
            local_updated_time,
            new_issue: new_issue.clone(),
            old_items,
            update_contents: activity_map[id].clone(),
        });
    }

    // refresh issues
    prev_data.issues.extend(new_issues);

    Ok(updates)
}

pub fn save_purged_data(
    args: &crate::cli_args::RedmineArgs,
    prev_data: &mut RedmineData,
) -> anyhow::Result<()> {
    let prev_data_path = &args.prev_redmine_data;
    let last_updated_date = prev_data
        .issues
        .values()
        .map(|i| {
            chrono::DateTime::parse_from_rfc3339(i["updated_on"].as_str().unwrap())
                .unwrap()
                .with_timezone(&chrono::Utc)
        })
        .max();
    if let Some(last_updated_date) = last_updated_date {
        prev_data.prev_date = last_updated_date.into();
    }
    // Remove old issues (and closed issues)
    if chrono::DateTime::<chrono::Utc>::MIN_UTC < prev_data.prev_date {
        let old_date = chrono::DateTime::<chrono::Utc>::from(
            prev_data.prev_date - humantime::parse_duration("50day").unwrap(),
        );
        prev_data.issues = prev_data
            .issues
            .iter()
            .filter(|(_, issue)| {
                let updated_on =
                    chrono::DateTime::parse_from_rfc3339(issue["updated_on"].as_str().unwrap())
                        .unwrap()
                        .with_timezone(&chrono::Utc);
                let is_closed = issue["status"]["is_closed"].as_bool().unwrap();
                old_date < updated_on && !is_closed
            })
            .map(|(id, issue)| (*id, issue.clone()))
            .collect();
    }

    // Save to file
    let json = serde_json::to_string_pretty(prev_data)?;
    std::fs::write(prev_data_path, json)?;

    Ok(())
}

pub struct Project {
    pub id: u32,
    pub name_id: String,
    pub name: String,
}

pub fn get_projects(url: &str, api_key: &Option<String>) -> anyhow::Result<Vec<Project>> {
    let mut list = Vec::new();
    let mut offset = 0;
    let projects_api = format!("{}/projects.json", url);
    loop {
        let json = get_json_from_api(&projects_api, api_key)?;
        let projects = json["projects"].as_array().unwrap();
        for project in projects {
            let id = project["id"].as_u64().unwrap() as u32;
            let name_id = project["identifier"].as_str().unwrap();
            let name = project["name"].as_str().unwrap();
            list.push(Project {
                id,
                name_id: name_id.to_string(),
                name: name.to_string(),
            });
        }
        let total_count = json["total_count"].as_u64().unwrap() as u32;
        let limit = json["limit"].as_u64().unwrap() as u32;
        if offset + limit < total_count {
            offset += limit;
        } else {
            break;
        }
    }
    Ok(list)
}

pub fn get_users_map(url: &str, api_key: &Option<String>) -> anyhow::Result<HashMap<String, u32>> {
    let mut map = HashMap::new();
    let mut offset = 0;
    let users_api = format!("{}/users.json", url);
    loop {
        let json = get_json_from_api(&users_api, api_key)?;
        let users = json["users"].as_array().unwrap();
        for user in users {
            let id = user["id"].as_u64().unwrap() as u32;
            let name = user["login"].as_str().unwrap();
            map.insert(name.to_string(), id);
        }
        let total_count = json["total_count"].as_u64().unwrap() as u32;
        let limit = json["limit"].as_u64().unwrap() as u32;
        if offset + limit < total_count {
            offset += limit;
        } else {
            break;
        }
    }
    Ok(map)
}

fn get_page(url: &str, api_key: &Option<String>) -> anyhow::Result<String> {
    let request = ureq::get(url);
    let request = if let Some(api_key) = api_key {
        request.set("X-Redmine-API-Key", api_key)
    } else {
        request
    };
    let response = request.call();
    match response {
        Ok(res) => match res.into_string() {
            Ok(content) => Ok(content),
            Err(e) => {
                let msg = format!("Could not into string [{url}] with {e}");
                crate::log::error(&msg).category("redmine");
                Err(Error::Get(msg).into())
            }
        },
        Err(ureq::Error::Status(code, res)) => {
            let status = res.status_text().to_string();
            let msg = format!(
                "Could not get page [{url}] Error: Status={code} {status}, Response={}",
                res.into_string().unwrap_or_default()
            );
            crate::log::error(&msg).category("redmine");
            Err(Error::Get(msg).into())
        }
        Err(e) => {
            let msg = format!("Could not get page [{url}] with {e}");
            crate::log::error(&msg).category("redmine");
            Err(Error::Get(msg).into())
        }
    }
}

fn get_json_from_api(url: &str, api_key: &Option<String>) -> anyhow::Result<serde_json::Value> {
    let response = get_page(url, api_key)?;
    let json: serde_json::Value = serde_json::from_str(&response)?;
    Ok(json)
}

fn get_atom_feed(url: &str, atom_key: &Option<String>) -> anyhow::Result<feed_rs::model::Feed> {
    let url = if let Some(atom_key) = atom_key {
        if url.contains("?") {
            format!("{}&key={}", url, atom_key)
        } else {
            format!("{}?key={}", url, atom_key)
        }
    } else {
        url.to_string()
    };
    let response = get_page(&url, &None)?;
    let feed = feed_rs::parser::parse(response.as_bytes())?;
    Ok(feed)
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("Get error. {0}")]
    Get(String),
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RedmineData {
    prev_date: chrono::DateTime<chrono::Utc>,
    issues: HashMap<u64, serde_json::Value>,
}

fn get_activities(
    prev_date: chrono::DateTime<chrono::Utc>,
    url: &str,
    atom_key: &Option<String>,
    filter: &crate::cli_args::FilterArgs,
    max_content_length: usize,
) -> anyhow::Result<HashMap<u64, Vec<UpdateContent>>> {
    // Get URL generation
    let mut activity_atoms = Vec::new();
    if let Some(target_project) = &filter.target_project {
        for project in target_project {
            activity_atoms.push(format!("{}/projects/{}/activity.atom", url, project));
        }
    } else {
        activity_atoms.push(format!("{}/activity.atom", url));
    }
    // If there is a User filter, add a query
    if let Some(users) = &filter.user {
        let org = activity_atoms;
        activity_atoms = Vec::new();
        for atom in org {
            for user in users {
                activity_atoms.push(format!("{}?user_id={}", atom, user));
            }
        }
    }
    // Get activities
    let mut activity_map = HashMap::<u64, Vec<UpdateContent>>::new();
    let prev_date = chrono::DateTime::<chrono::Utc>::from(prev_date);
    let id_re = regex::Regex::new(r".+/(\d+)").unwrap(); // get numbers from the last /
    let html_re = regex::Regex::new(r"<[^>]*?>|\n").unwrap();
    for atom in activity_atoms {
        let feed = get_atom_feed(&atom, atom_key)?;
        'entry: for entry in feed.entries {
            // Ignore projects
            if let Some(ignore_projects) = &filter.ignore_project {
                for project in ignore_projects {
                    if entry
                        .title
                        .as_ref()
                        .unwrap()
                        .content
                        .starts_with(&format!("{} - ", project))
                    {
                        continue 'entry;
                    }
                }
            }
            // If the date is older than the previous date, ignore it.
            if let Some(updated) = &entry.updated {
                if updated <= &prev_date {
                    continue;
                }
            }

            // The issue number is the last / after id,
            let id = id_re.captures(&entry.id).unwrap()[1]
                .parse::<u64>()
                .unwrap();
            let content = entry.content.map_or(None, |c| c.body);

            // Group by id
            if let Some(content) = content {
                let mut content = html_re.replace_all(&content, "").trim().to_string();
                if !content.is_empty() {
                    if max_content_length < content.chars().count() {
                        content = format!(
                            "{}...",
                            &content.chars().take(max_content_length).collect::<String>()
                        );
                    }
                    let update_content = UpdateContent {
                        author: entry.authors[0].name.clone(),
                        content,
                    };
                    if let Some(value) = activity_map.get_mut(&id) {
                        // Since Atom is in descending order, insert it at the beginning to sort it in ascending order.
                        value.insert(0, update_content);
                    } else {
                        activity_map.insert(id, vec![update_content]);
                    }
                } else if !activity_map.contains_key(&id) {
                    activity_map.insert(id, vec![]);
                }
            } else if !activity_map.contains_key(&id) {
                activity_map.insert(id, vec![]);
            }
        }
    }

    Ok(activity_map)
}

fn get_issues(
    url: &str,
    api_key: &Option<String>,
    updated_ids: &[u64],
) -> anyhow::Result<HashMap<u64, serde_json::Value>> {
    let mut issues = HashMap::new();
    let mut offset = 0;
    let issues_api = format!(
        "{}/issues.json?status_id=*&issue_id={}",
        url,
        updated_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );
    loop {
        let json = get_json_from_api(&issues_api, api_key)?;
        json["issues"].as_array().unwrap().iter().for_each(|issue| {
            issues.insert(issue["id"].as_u64().unwrap(), issue.clone());
        });
        let total_count = json["total_count"].as_u64().unwrap() as u32;
        let limit = json["limit"].as_u64().unwrap() as u32;
        if offset + limit < total_count {
            offset += limit;
        } else {
            break;
        }
    }
    Ok(issues)
}

fn get_updated_items(
    old_issue: &Option<&serde_json::Value>,
    new_issue: &serde_json::Value,
) -> anyhow::Result<serde_json::Value> {
    // Extract fields in old_issue that are different from new_issue
    let mut updated_items = serde_json::Map::new();
    for (key, value) in new_issue.as_object().unwrap() {
        if let Some(old_issue) = old_issue {
            if let Some(old_value) = old_issue.get(key) {
                if value != old_value {
                    updated_items.insert(key.to_string(), old_value.clone());
                }
            }
        }
        if updated_items.get(key).is_none() {
            updated_items.insert(key.to_string(), serde_json::Value::Null);
        }
    }
    if let Some(old_issue) = old_issue {
        for (key, value) in old_issue.as_object().unwrap() {
            if updated_items.get(key).is_none() {
                updated_items.insert(key.to_string(), value.clone());
            }
        }
    }
    Ok(serde_json::Value::Object(updated_items))
}
