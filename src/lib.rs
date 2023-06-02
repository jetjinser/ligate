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

    let token = env::var("LIGA_TOKEN");
    let client_id = env::var("client_id");
    let secret_key = env::var("secret_key");

    let liga = if let Ok(t) = token {
        Liga::from_token(t)
    } else {
        let client_id = client_id.unwrap();
        let secret_key = secret_key.unwrap();
        Liga::from_client(client_id, secret_key)
    };

    listen_to_event(&login, &owner, &repo, events, |payload| async {
        handle(&login, &owner, &repo, liga, payload).await
    })
    .await;
}

async fn handle(login: &GithubLogin, owner: &str, repo: &str, liga: Liga, payload: EventPayload) {
    if let EventPayload::IssueCommentEvent(e) = payload {
        let title = e.issue.title;
        let body = e.issue.body.unwrap_or_default();
        let comment = e.comment.body.unwrap_or_default();

        if !comment.starts_with("liga") {
            return;
        }

        let octo = get_octo(login);

        let issue_type_id = 98537026;
        let data = serde_json::json!({
            "summary": title,
            "description": body,
            "status": 98536908,
        });
        let project_id = 98536876;
        let res: serde_json::Value = liga.issue().add(issue_type_id, data, project_id);

        let id = &res["data"]["id"].as_u64();
        let number = e.issue.number;
        if let Some(i) = id {
            let url = format!("https://ligai.cn/app/work/table?pid={project_id}&issueid={i}");
            let body = format!(
                "You just created a new issue!\nplease visit [LigaAI]({}) to check it.",
                url
            );
            _ = octo.issues(owner, repo).create_comment(number, body).await;
        } else {
            _ = octo
                .issues(owner, repo)
                .create_comment(
                    number,
                    format!("failed...\n{:?}", serde_json::to_string(&res)),
                )
                .await;
        }
    }
}
