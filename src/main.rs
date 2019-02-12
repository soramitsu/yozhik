#[macro_use] extern crate serde_derive;

use log::info;
use actix_web::{App, Responder, HttpRequest, HttpResponse, FutureResponse, Error, AsyncResponder};
use actix_web::server;
use actix_web::http::Method;
use actix_web::middleware::Logger;
use actix_web::error::ErrorBadRequest;
use futures::future::{self, Future};

pub mod config;
pub mod build_info;
pub mod middleware;
pub mod jira;
pub mod github;

use self::config::*;
use self::github::GithubHookData;

fn github_hook(req: HttpRequest, body: String) -> FutureResponse<&'static str> {
    let data = middleware::validate(&req, body.as_bytes())
        .and_then(|_| serde_json::from_str(body.as_str()).map_err(ErrorBadRequest));
    let response = |data: GithubHookData| match (&data.issue, &data.action) {
        (&Some(ref i), &Some(ref action)) if action == "opened" => {
            let full_name = data.repository.full_name.clone();
            let number = i.number;
            let cloned = i.clone();
            let jira_code = get_param(full_name.as_str(), "jira-project-key");
            let jira_issue_type = get_param(full_name.as_str(), "jira-issue-type-id")
                .unwrap_or(JIRA_ISSUE_ID_DEFAULT.clone());
            github::close_issue(&full_name, number)
                .map_err(Error::from)
                .and_then(move |_| future::ok(jira_code).and_then(move |e| e.map(|code|
                    jira::open_issue(
                        code,
                        cloned.title.clone(),
                        cloned.body.clone(),
                        jira_issue_type,
                    )
                )))
                .map(|e| e.and_then(|k| k))
                .and_then(move |e| {
                    let comment = if let (Some(ref jira_issue), Some(api)) = (e, &*JIRA_API) {
                        format!("{}\n[Jira]: {}browse/{}", *COMMENT, *api, jira_issue.key)
                    } else {
                        COMMENT.clone()
                    };
                    github::comment_issue(&full_name, number, comment).map_err(Error::from)
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
