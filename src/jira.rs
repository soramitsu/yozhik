use log::info;
use actix_web::client::{self, ClientResponse, SendRequestError};
use actix_web::{Error, HttpMessage};
use base64::encode;
use futures::future::{self, Future};

use super::config::{JIRA_API, JIRA_EMAIL, JIRA_TOKEN};

#[derive(Serialize, Debug)]
struct JiraIssue {
    pub fields: JiraIssueFields,
}

#[derive(Serialize, Debug)]
pub struct JiraIssueFields {
    pub project: JiraProject,
    pub summary: String,
    pub description: String,
    pub issuetype: JiraIssueType,
    pub labels: Vec<String>,
}

#[derive(Serialize, Debug)]
pub struct JiraProject {
    pub key: String,
}

#[derive(Serialize, Debug)]
pub struct JiraIssueType {
    pub id: String,
}

#[derive(Deserialize, Debug)]
pub struct JiraIssueCreated {
    pub key: String,
}


fn open_issue_raw(query: JiraIssue) -> impl Future<Item=Option<ClientResponse>, Error=SendRequestError> {
    future::ok(if let (Some(i), Some(j), Some(k)) = (&*JIRA_EMAIL, &*JIRA_TOKEN, &*JIRA_API) {
        Some((i, j, k))
    } else {
        None
    }).and_then(|a| a.map(|(i, j, k)| {
        let token = encode(&format!("{}:{}", i, j));
        let url = format!("{}rest/api/2/issue", k);
        let query_string = serde_json::to_string(&query);
        client::post(url.clone())
            .header("Authorization", format!("Basic {}", token))
            .json(query)
            .unwrap().send()
            .then(move |res| {
                info!("Sent open issue request to Jira: {:?}\nURL: {}\nQuery: {:?}", res, url, query_string);
                res
            })
    }))
}

pub fn open_issue(
    project_key: String,
    issue_title: String,
    issue_description: String,
    issue_type_id: String,
) -> impl Future<Item=Option<JiraIssueCreated>, Error=Error> {
    let query: JiraIssue = JiraIssue { fields: JiraIssueFields {
        project: JiraProject { key: project_key },
        summary: issue_title,
        description: issue_description,
        issuetype: JiraIssueType { id: issue_type_id.to_string() },
        labels: vec!["github".to_string()],
    }};
    open_issue_raw(query)
        .map_err(Error::from)
        .and_then(|e| e.map(|resp| {
            resp.body().map_err(Error::from).and_then(|body| {
                serde_json::from_slice::<JiraIssueCreated>(&body).map_err(Error::from)
            })
        }))
}