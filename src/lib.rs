use std::env;

use github_flows::{get_octo, listen_to_event, EventPayload, GithubLogin};
use ligab::Liga;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    let login = GithubLogin::Default;
    let owner = env::var("owner").unwrap_or("jetjinser".to_string());
    let repo = env::var("repo").unwrap_or("fot".to_string());
    let events = vec!["issue_comment"];

    let token = env::var("LIGA_TOKEN").unwrap();

    listen_to_event(&login, &owner, &repo, events, |payload| async {
        handle(&login, &owner, &repo, token, payload).await
    })
    .await;
}

async fn handle(
    login: &GithubLogin,
    owner: &str,
    repo: &str,
    token: String,
    payload: EventPayload,
) {
    if let EventPayload::IssueCommentEvent(e) = payload {
        let title = e.issue.title;
        let comment = e.comment.body_text.unwrap_or_default();

        let octo = get_octo(login);

        let number = e.issue.number;

        _ = octo.issues(owner, repo).create_comment(number, "ok").await;

        if !comment.starts_with("liga") {
            return;
        }

        let liga = Liga::from_token(token);

        let issue_type_id = 98537026;
        let data = serde_json::json!({
            "summary": title,
            "description": comment,
            "status": 98536908,
        });
        let project_id = 98536876;
        let res: serde_json::Value = liga.issue().add(issue_type_id, data, project_id);

        let id = &res["data"]["id"].as_u64();

        if let Some(i) = id {
            let url = format!("https://ligai.cn/app/work/table?pid={project_id}&issueid={i}");
            let body = format!(
                "You just created issue: {}\nplease visit {} to check it.",
                i, url
            );
            _ = octo.issues(owner, repo).create_comment(number, body).await;
        } else {
            _ = octo
                .issues(owner, repo)
                .create_comment(number, "failed...")
                .await;
        }
    }
}
