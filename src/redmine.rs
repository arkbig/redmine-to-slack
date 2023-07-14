// Copyright © ArkBig

pub struct Ticket {
    subscribe_url: String,
}

pub fn subscribe(
    url: &String,
    prev_date: &chrono::DateTime<chrono::Utc>,
) -> anyhow::Result<Vec<Ticket>> {
    let xml = get_page(url)?;
    let feed = feed_rs::parser::parse(xml.as_bytes())?;
    for (entry in feed.entries) {
        
    }
}

fn get_page(url: &str) -> anyhow::Result<String> {
    let response = ureq::get(url).call();
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

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("Get error. {0}")]
    Get(String),
}
