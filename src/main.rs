#[macro_use] extern crate serde_derive;

use log::{info, error};
use actix_web::{App, Responder, HttpRequest, HttpResponse, FutureResponse, Error, AsyncResponder};
use actix_web::server;
use actix_web::client::{self, ClientResponse, SendRequestError};
use actix_web::http::Method;
use actix_web::middleware::Logger;
use actix_web::error::ErrorBadRequest;
use futures::future::Future;

pub mod config;
pub mod build_info;
pub mod middleware;

use self::config::*;

fn change_issue_url(full_name: &str, number: i128) -> String {
    format!("{}repos/{}/issues/{}", GITHUB_API, full_name, number)
}

fn send_issue_comment_url(full_name: &str, number: i128) -> String {
    format!("{}repos/{}/issues/{}/comments", GITHUB_API, full_name, number)
}

#[derive(Serialize, Debug)]
struct ChangeIssue {
    state: String,
}

#[derive(Serialize, Debug)]
struct PostComment {
    body: &'static str,
}

#[derive(Deserialize, Debug)]
struct GithubHookData {
    action: Option<String>,
    issue: Option<Issue>,
    repository: Repository,
}

#[derive(Deserialize, Debug)]
struct Issue {
    url: String,
    repository_url: String,
    id: i128,
    number: i128,
    state: String,
}

#[derive(Deserialize, Debug)]
struct Repository {
    id: i128,
    node_id: String,
    url: String,
    full_name: String,
}


fn close_issue(full_name: &str, number: i128) -> impl Future<Item=ClientResponse, Error=SendRequestError> {
    let url = change_issue_url(&full_name, number);
    client::post(url.clone())
        .header("Authorization", format!("token {}", TOKEN.as_str()))
        .json(ChangeIssue { state: "closed".to_string() })
        .unwrap().send()
        .then(move |res| {
            info!("Close issue request complete: {:?}\nURL: {}", res, url);
            res
        })
}

fn comment_issue(full_name: &str, number: i128) -> impl Future<Item=ClientResponse, Error=SendRequestError> {
    let url = send_issue_comment_url(&full_name, number);
    client::post(url.clone())
        .header("Authorization", format!("token {}", TOKEN.as_str()))
        .json(PostComment { body: COMMENT.as_str() })
        .unwrap().send()
        .then(move |res| {
            info!("Comment issue request complete: {:?}\nURL: {}", res, url);
            res
        })
}


fn github_hook(req: HttpRequest, body: String) -> FutureResponse<&'static str> {
    let data = middleware::validate(&req, body.as_bytes())
        .and_then(|_| serde_json::from_str(body.as_str()).map_err(ErrorBadRequest));
    let response = |data: GithubHookData| match (&data.issue, &data.action) {
        (&Some(ref i), &Some(ref action)) if action == "opened" => {
            let full_name = data.repository.full_name.clone();
            let number = i.number;
            close_issue(&full_name, number)
                .and_then(move |_| comment_issue(&full_name, number))
                .map_err(|e| {
                    error!("Failed to handle opened issue, error: {:?}", e);
                    Error::from(e)
                })
                .map(|_| "")
                .responder()
        }
        _ => futures::finished("").responder()
    };
    futures::done(data).and_then(response).responder()
}

fn info(_: HttpRequest) -> impl Responder {
    HttpResponse::Ok().json(build_info::INFO)
}

fn app() -> App<()> {
    App::new()
        .middleware(Logger::default())
        .route("/info", Method::GET, info)
        .route("/github", Method::POST, github_hook)
}

fn main() {
    env_logger::init();
    info!("Setting comment to: {}", COMMENT.as_str());
    println!("Your Github webhook key is: {}", WEBHOOK_KEY.as_str());
    server::new(app).bind(BIND_ADDRESS.as_str())
        .expect("Failed to start server")
        .run();
}
