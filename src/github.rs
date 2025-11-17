use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ContributionDay {
    pub date: String,
    pub contribution_count: u32,
}

#[derive(Deserialize)]
pub struct ContributionWeek {
    pub contribution_days: Vec<ContributionDay>,
}

#[derive(Deserialize)]
pub struct ContributionCalendar {
    pub weeks: Vec<ContributionWeek>,
}

#[derive(Deserialize)]
pub struct ContributionsCollection {
    pub contribution_calendar: ContributionCalendar,
}

#[derive(Deserialize)]
pub struct User {
    pub contributions_collection: ContributionsCollection,
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

    let res = client
        .post("https://api.github.com/graphql")
        .bearer_auth(token)
        .header("User-Agent", "rust-contribution-checker")
        .json(&serde_json::json!({ "query": query }))
        .send()
        .await?
        .json::<GraphQLResponse>()
        .await?;

    let today = chrono::Local::now().format("%Y-%m-%d").to_string();

    let days = res
        .data
        .user
        .contributions_collection
        .contribution_calendar
        .weeks
        .into_iter()
        .flat_map(|w| w.contribution_days)
        .collect::<Vec<_>>();

    for d in days {
        if d.date == today {
            return Ok(d.contribution_count);
        }
    }

    Ok(0)
}
