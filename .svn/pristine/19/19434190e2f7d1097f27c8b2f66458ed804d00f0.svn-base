use anyhow::{Context, Result, anyhow};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use url::Url;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct GithubReleaseAsset {
    name: String,
    browser_download_url: String,
    content_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct GithubRelease {
    tag_name: String,
    assets: Vec<GithubReleaseAsset>,
    assets_url: String,
}

pub struct ModuleInstaller {
    modules_dir: String,
}

impl ModuleInstaller {
    pub fn new(modules_dir: &str) -> Self {
        Self {
            modules_dir: modules_dir.to_string(),
        }
    }

    pub async fn install_from_github(&self, github_url: &str) -> Result<String> {
        let (owner, repo) = self.parse_github_url(github_url)?;

        let release = self.get_latest_release(&owner, &repo).await?;

        let asset = self.find_so_asset(&release)?;

        let file_path = self.download_asset(&asset).await?;

        Ok(file_path)
    }

    fn parse_github_url(&self, github_url: &str) -> Result<(String, String)> {
        let url = Url::parse(github_url)?;

        if url.host_str() != Some("github.com") {
            return Err(anyhow!("Not a GitHub URL"));
        }

        let path_segments: Vec<&str> = url
            .path_segments()
            .ok_or(anyhow!("Invalid URL path"))?
            .collect();

        if path_segments.len() < 2 {
            return Err(anyhow!("Invalid GitHub repository URL"));
        }

        let owner = path_segments[0].to_string();
        let repo = path_segments[1]
            .to_string()
            .trim_end_matches(".git")
            .to_string();

        Ok((owner, repo))
    }

    async fn get_latest_release(&self, owner: &str, repo: &str) -> Result<GithubRelease> {
        let client = reqwest::Client::new();
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static("zero-core-ide-module-installer"),
        );

        let url = format!(
            "https://api.github.com/repos/{}/{}/releases/latest",
            owner, repo
        );

        let response = client.get(&url).headers(headers).send().await?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "GitHub API request failed with status: {}",
                response.status()
            ));
        }

        let json = response.text().await?;
        serde_json::from_str(&json).context("Failed to parse json")
    }

    fn find_so_asset(&self, release: &GithubRelease) -> Result<GithubReleaseAsset> {
        for asset in &release.assets {
            if asset.name.ends_with(".so") {
                return Ok(asset.clone());
            }
        }

        Err(anyhow!("No .so file found in release {}", release.tag_name))
    }

    async fn download_asset(&self, asset: &GithubReleaseAsset) -> Result<String> {
        let client = reqwest::Client::new();

        let dest_path = Path::new(&self.modules_dir).join(&asset.name);

        fs::create_dir_all(&self.modules_dir)?;

        let response = client.get(&asset.browser_download_url).send().await?;

        if !response.status().is_success() {
            return Err(anyhow!("Failed to download asset: {}", response.status()));
        }

        let content = response.bytes().await?;
        fs::write(&dest_path, content)?;

        Ok(dest_path.to_string_lossy().to_string())
    }
}
