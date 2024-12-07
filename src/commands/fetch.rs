use super::texts::{FETCH_GITHUB_ERROR, READ_JSON_ERROR};
use formats::*;
use std::sync::OnceLock;
use structs::*;
use tokio::sync::OnceCell;
use tokio::try_join;

mod formats;
mod structs;

const JSON: &str = include_str!("../../configs/config.json");

// Once statics

static CONFIG: OnceLock<Option<Config>> = OnceLock::new();
static PROMPT: OnceLock<String> = OnceLock::new();
static GITHUB: OnceCell<String> = OnceCell::const_new();
static REPOS: OnceCell<String> = OnceCell::const_new();
static CONTACTS: OnceLock<String> = OnceLock::new();

// Once Functions

fn read_config() -> Option<Config> {
    CONFIG
        .get_or_init(|| match serde_json::from_str::<Config>(JSON) {
            Ok(config) => Some(config),
            Err(_) => None,
        })
        .clone()
}

pub fn get_prompt() -> String {
    PROMPT
        .get_or_init(|| match read_config() {
            Some(config) => format!("{}@termfolio~$ ", config.github),
            _ => String::from("user@termfolio~$ "),
        })
        .clone()
}

pub fn get_about() -> String {
    match read_config() {
        Some(config) => format_about(config.about),
        _ => String::from(READ_JSON_ERROR),
    }
}

pub async fn get_github() -> String {
    GITHUB.get_or_init(fetch_github).await;

    match GITHUB.get() {
        Some(user) => user.to_string(),
        _ => String::from(FETCH_GITHUB_ERROR),
    }
}

pub async fn get_repos() -> String {
    REPOS.get_or_init(fetch_repos).await;

    match REPOS.get() {
        Some(repos) => repos.to_string(),
        _ => String::from(FETCH_GITHUB_ERROR),
    }
}

pub fn get_contacts() -> &'static String {
    CONTACTS.get_or_init(|| match read_config() {
        Some(config) => format_links(config.links),
        _ => String::from(READ_JSON_ERROR),
    })
}

// Fetch functions

async fn fetch_github() -> String {
    match read_config() {
        Some(config) => {
            let info_url = format!("https://api.github.com/users/{}", config.github);
            let stats_url = format!(
                "https://api.github-star-counter.workers.dev/user/{}",
                config.github
            );

            match try_join!(async { reqwest::get(&info_url).await }, async {
                reqwest::get(&stats_url).await
            }) {
                Ok((info_response, stats_response)) => {
                    if info_response.status().is_success() && stats_response.status().is_success() {
                        let profile = Profile {
                            username: config.github,
                            langs: config.about.langs,
                            info: info_response.json().await.unwrap(),
                            stats: stats_response.json().await.unwrap(),
                        };

                        format_profile(profile)
                    } else {
                        String::from(FETCH_GITHUB_ERROR)
                    }
                }
                Err(_) => String::from(FETCH_GITHUB_ERROR),
            }
        }
        None => String::from(READ_JSON_ERROR),
    }
}

async fn fetch_repos() -> String {
    match read_config() {
        Some(config) => {
            let repos_url = format!("https://pinned.berrysauce.dev/get/{}", config.github);

            match reqwest::get(&repos_url).await {
                Ok(response) => {
                    let repos: Vec<Repository> = response.json().await.unwrap();
                    format_repos(&repos)
                }
                Err(_) => String::from(FETCH_GITHUB_ERROR),
            }
        }
        None => String::from(READ_JSON_ERROR),
    }
}
