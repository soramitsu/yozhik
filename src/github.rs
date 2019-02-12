use log::info;

use actix_web::client::{self, ClientResponse, SendRequestError};
use futures::future::Future;

use super::config::*;

fn url_github_change_issue(full_name: &str, number: i128) -> String {
    format!("{}repos/{}/issues/{}", GITHUB_API, full_name, number)
}

fn url_github_send_issue_comment(full_name: &str, number: i128) -> String {
    format!("{}repos/{}/issues/{}/comments", GITHUB_API, full_name, number)
}

#[derive(Serialize, Debug)]
struct GithubChangeIssue {
    state: String,
}

#[derive(Serialize, Debug)]
struct GithubPostComment {
    body: String,
}

#[derive(Deserialize, Debug)]
pub struct GithubHookData {
    pub action: Option<String>,
    pub issue: Option<GithubIssue>,
    pub repository: GithubRepository,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GithubIssue {
    pub url: String,
    pub repository_url: String,
    pub id: i128,
    pub number: i128,
    pub state: String,
    pub title: String,
    pub body: String,
    pub html_url: String,
}

#[derive(Deserialize, Debug)]
pub struct GithubRepository {
    pub id: i128,
    pub node_id: String,
    pub url: String,
    pub full_name: String,
}

pub fn close_issue(full_name: &str, number: i128) -> impl Future<Item=ClientResponse, Error=SendRequestError> {
    let url = url_github_change_issue(&full_name, number);
    client::post(url.clone())
        .header("Authorization", format!("token {}", TOKEN.as_str()))
        .json(GithubChangeIssue { state: "closed".to_string() })
        .unwrap().send()
        .then(move |res| {
            info!("Close issue request complete: {:?}\nURL: {}", res, url);
            res
        })
}

pub fn comment_issue(full_name: &str, number: i128, body: String) -> impl Future<Item=ClientResponse, Error=SendRequestError> {
    let url = url_github_send_issue_comment(&full_name, number);
    client::post(url.clone())
        .header("Authorization", format!("token {}", TOKEN.as_str()))
        .json(GithubPostComment { body })
        .unwrap().send()
        .then(move |res| {
            info!("Comment issue request complete: {:?}\nURL: {}", res, url);
            res
        })
}
