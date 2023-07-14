use dotenvy_macro::dotenv;
use reqwest::header;
use serde::Deserialize;

const ORGANIZATION: &str = "edf-re";
const PERSONAL_ACCESS_TOKEN: &str = dotenv!("PERSONAL_ACCESS_TOKEN");

#[derive(Deserialize)]
struct Release {
    name: Option<String>,
    published_at: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Repository {
    name: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .user_agent("github-releases-cli")
        .default_headers({
            let mut headers = header::HeaderMap::new();
            headers.insert(
                header::AUTHORIZATION,
                header::HeaderValue::from_str(&format!("Bearer {}", PERSONAL_ACCESS_TOKEN))?,
            );
            headers.insert(
                header::ACCEPT,
                header::HeaderValue::from_static("application/vnd.github.v3+json"),
            );
            headers
        })
        .build()?;

    let mut page = 1;
    let mut all_repositories = Vec::new();

    loop {
        let url = format!(
            "https://api.github.com/orgs/{}/repos?page={}&per_page=100",
            ORGANIZATION, page
        );

        let response = client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(format!("Request failed with status code: {}", response.status()).into());
        }

        let json_response = response.text().await?;
        let repositories: Vec<Repository> = serde_json::from_str(&json_response)?;

        if repositories.is_empty() {
            break;
        }

        all_repositories.extend(repositories);
        page += 1;
    }

    let mut releases = Vec::new();

    for repo in all_repositories {
        if let Some(name) = repo.name {
            println!("Fetching releases for {}", name);
            let releases_url = format!(
                "https://api.github.com/repos/{}/{}/releases",
                ORGANIZATION, name
            );

            let response = client.get(&releases_url).send().await?;

            if !response.status().is_success() {
                return Err(
                    format!("Request failed with status code: {}", response.status()).into(),
                );
            }

            let json_response = response.text().await?;
            let repo_releases: Vec<Release> = serde_json::from_str(&json_response)?;

            if let Some(last_release) = repo_releases.first() {
                releases.push((
                    name,
                    last_release.name.clone(),
                    last_release.published_at.clone(),
                ));
            }
        } else {
            println!("Skipping repository with no name");
        }
    }

    releases.sort_by(|a, b| b.1.cmp(&a.1));

    println!("\n");
    println!("********************************************");
    println!("********** Latest Releases by Date *********");
    println!("********************************************");
    println!("\n");

    for (name, release_name, published_at) in releases {
        if let Some(release_date) = published_at {
            println!(
                "{:<60} \t {:<30} \t{}",
                name,
                release_date,
                release_name.unwrap_or("No release name".to_string()),
            );
        }
    }

    Ok(())
}
