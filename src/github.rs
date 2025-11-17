#![allow(non_snake_case)]

use std::env;

use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;
use tracing::{error, info};

#[derive(Deserialize)]
pub struct ContributionDay {
    pub date: String,
    pub contributionCount: u32,
}

#[derive(Deserialize)]
pub struct ContributionWeek {
    pub contributionDays: Vec<ContributionDay>,
}

#[derive(Deserialize)]
pub struct ContributionCalendar {
    pub weeks: Vec<ContributionWeek>,
}

#[derive(Deserialize)]
pub struct ContributionsCollection {
    pub contributionCalendar: ContributionCalendar,
}

#[derive(Deserialize)]
pub struct User {
    pub contributionsCollection: ContributionsCollection,
}

#[derive(Deserialize)]
pub struct Data {
    pub user: User,
}

#[derive(Deserialize)]
pub struct GraphQLResponse {
    pub data: Data,
}

pub async fn get_today_contribution(client: &Client, token: &str, username: &str) -> Result<u32> {
    let query = format!(
        r#"
        query {{
          user(login: "{username}") {{
            contributionsCollection {{
              contributionCalendar {{
                weeks {{
                  contributionDays {{
                    date
                    contributionCount
                  }}
                }}
              }}
            }}
          }}
        }}
        "#
    );

    info!("📡 REQUEST → GitHub GraphQL (get_today_contribution)");

    let raw = client
        .post("https://api.github.com/graphql")
        .bearer_auth(token)
        .header("User-Agent", "rust-contrib-checker")
        .json(&serde_json::json!({ "query": query }))
        .send()
        .await;

    let res = match raw {
        Ok(r) => r,
        Err(e) => {
            error!("❌ GitHub request error: {:?}", e);
            return Ok(0);
        }
    };

    let text = res.text().await.unwrap_or_default();

    if env::var("MODE").unwrap_or("NORMAL".into()) == "TEST" {
        info!("📥 RESPONSE ← GitHub: {}", text);
    }

    let parsed: GraphQLResponse = serde_json::from_str(&text).unwrap_or_else(|e| {
        error!("❌ JSON parse error: {:?}", e);
        panic!("GitHub JSON parsing failed");
    });

    let today = chrono::Local::now().format("%Y-%m-%d").to_string();

    for w in parsed
        .data
        .user
        .contributionsCollection
        .contributionCalendar
        .weeks
    {
        for d in w.contributionDays {
            if d.date == today {
                return Ok(d.contributionCount);
            }
        }
    }

    Ok(0)
}
