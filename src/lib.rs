use std::env;

use github_flows::{listen_to_event, EventPayload, GithubLogin};
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
        handle(token, payload).await
    })
    .await;
}

async fn handle(token: String, payload: EventPayload) {
    if let EventPayload::IssueCommentEvent(e) = payload {
        let title = e.issue.title;
        let review = e.issue.body_text.unwrap_or_default();

        let liga = Liga::from_token(token);

        let issue_type_id = 98537026;
        let data = serde_json::json!({
            "summary": title,
            "description": review,
            "status": 98536908,
        });
        let project_id = 98536876;
        liga.issue()
            .add::<_, serde_json::Value>(issue_type_id, data, project_id);
    }
}
