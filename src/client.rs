use crate::types::*;

const DEFAULT_BASE_URL: &str = "https://barcodefyi.com/api";

/// Async client for the BarcodeFYI API.
pub struct Client {
    base_url: String,
    http: reqwest::Client,
}

impl Client {
    /// Creates a new client with the default base URL.
    pub fn new() -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.to_string(),
            http: reqwest::Client::new(),
        }
    }

    /// Creates a new client with a custom base URL.
    pub fn with_base_url(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            http: reqwest::Client::new(),
        }
    }

    async fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T, BarcodeFyiError> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self.http.get(&url).send().await?;
        if !resp.status().is_success() {
            return Err(BarcodeFyiError::Api {
                status: resp.status().as_u16(),
                body: resp.text().await.unwrap_or_default(),
            });
        }
        Ok(resp.json().await?)
    }

    /// Search across barcode symbologies, standards, and glossary terms.
    pub async fn search(&self, query: &str) -> Result<SearchResult, BarcodeFyiError> {
        let encoded = urlencoding(query);
        self.get(&format!("/search/?q={}", encoded)).await
    }

    /// Get details for a barcode symbology by slug.
    pub async fn symbology(&self, slug: &str) -> Result<SymbologyDetail, BarcodeFyiError> {
        self.get(&format!("/symbology/{}/", slug)).await
    }

    /// Get details for a barcode family by slug.
    pub async fn family(&self, slug: &str) -> Result<FamilyDetail, BarcodeFyiError> {
        self.get(&format!("/family/{}/", slug)).await
    }

    /// Get details for a barcode standard by slug.
    pub async fn standard(&self, slug: &str) -> Result<StandardDetail, BarcodeFyiError> {
        self.get(&format!("/standard/{}/", slug)).await
    }

    /// Get details for a barcode component by slug.
    pub async fn component(&self, slug: &str) -> Result<ComponentDetail, BarcodeFyiError> {
        self.get(&format!("/component/{}/", slug)).await
    }

    /// Get a glossary term by slug.
    pub async fn glossary_term(&self, slug: &str) -> Result<GlossaryTerm, BarcodeFyiError> {
        self.get(&format!("/glossary/{}/", slug)).await
    }

    /// Compare two barcode symbologies.
    pub async fn compare(&self, slug_a: &str, slug_b: &str) -> Result<CompareResult, BarcodeFyiError> {
        self.get(&format!("/compare/?a={}&b={}", slug_a, slug_b)).await
    }

    /// Get a random barcode symbology.
    pub async fn random(&self) -> Result<SymbologyDetail, BarcodeFyiError> {
        self.get("/random/").await
    }

    /// Get details for a barcode industry by slug.
    pub async fn industry(&self, slug: &str) -> Result<IndustryDetail, BarcodeFyiError> {
        self.get(&format!("/industry/{}/", slug)).await
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

fn urlencoding(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
            ' ' => "+".to_string(),
            _ => format!("%{:02X}", c as u32),
        })
        .collect()
}
