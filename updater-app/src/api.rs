use anyhow::Context;
use ureq::Error;

#[derive(Debug, Clone)]
pub struct Api {
    base_url: String,
}

impl Default for Api {
    fn default() -> Self {
        Api {
            base_url: "http://localhost:7313".to_string(),
        }
    }
}

impl Api {
    // GET http://baseUrl/releases?release=1
    pub fn get_releases(&self, release_id: u64) -> Result<Vec<ReleaseDetails>, Error> {
        let url = format!("{}/releases", self.base_url);
        let response = ureq::get(url.as_str())
            .query("release", release_id.to_string().as_str())
            .call()?
            .into_json::<GetAllReleasesResponse>()?;
        return Ok(response.0);
    }
    // GET http://baseUrl/files?file=someFile.txt
    pub fn get_file(&self, file: &str) -> anyhow::Result<Vec<u8>> {
        let url = format!("{}/files", self.base_url);
        let response = ureq::get(url.as_str())
            .query("file", file)
            .call()
            .context("Failed to get file")?;
        let mut bytes: Vec<u8> = Vec::new();
        response.into_reader().read_to_end(&mut bytes)?;
        Ok(bytes)
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct GetAllReleasesResponse(pub Vec<ReleaseDetails>);

#[derive(Debug, serde::Deserialize)]
pub struct ReleaseDetails {
    pub id: u64,
    pub files: Vec<ReleaseFileDetails>,
}

#[derive(Debug, serde::Deserialize)]
pub struct ReleaseFileDetails {
    pub path: String,
    pub sha1: String,
}
