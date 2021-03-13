use anyhow::{anyhow, Result};
use changelog::{ChangeLog, Configuration};
use serde_derive::Serialize;
use std::env;
use tinytemplate::TinyTemplate;

#[derive(Serialize)]
struct Context {
    this_version: String,
    repo_url: String,
    commits_link: String,
    changelog: ChangeLog,
}

fn main() -> Result<()> {
    let mut args = env::args();
    let exe = args.next().expect("could not retrieve exe name");
    let previous_version = args.next().ok_or(anyhow!(
        "usage: {} <previous version tag> <next version tag> [repo base url]",
        exe
    ))?;
    let this_version = args.next().ok_or(anyhow!(
        "usage: {} <previous version tag> <next version tag> [repo base url]",
        exe
    ))?;
    let repo_url = args.next();

    let range = format!("{}..{}", previous_version, this_version);
    let (commits_link, repo_url) = if let Some(repo_url) = repo_url {
        let repo_url = repo_url.trim_end_matches('/');

        (
            format!(
                "{}/compare/{}...{}",
                repo_url, previous_version, this_version
            ),
            repo_url.to_string(),
        )
    } else {
        (String::from("#"), String::from("."))
    };

    let config = Configuration::from_yaml(include_str!("../../.changelog.yml"))?;
    let changelog = ChangeLog::from_range(&range, &config);

    let mut tt = TinyTemplate::new();
    tt.add_template(
        "changelog_template",
        include_str!("../../assets/changelog-template.md"),
    )?;

    let context = Context {
        this_version,
        repo_url,
        commits_link,
        changelog,
    };

    let rendered = tt.render("changelog_template", &context)?;
    println!("{}", rendered);

    Ok(())
}
