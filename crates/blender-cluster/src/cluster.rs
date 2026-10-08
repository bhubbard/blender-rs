use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub struct NodeInfo {
    pub name: String,
    pub ollama_url: String,
    pub zev_url: String,
}

#[derive(Serialize)]
struct OllamaRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    stream: bool,
    options: OllamaOptions,
}

#[derive(Serialize)]
struct OllamaOptions {
    num_predict: u32,
    temperature: f32,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: Option<String>,
    eval_count: Option<u64>,
}

#[derive(Clone)]
pub struct ClusterClient {
    client: Client,
    pub nodes: Vec<NodeInfo>,
}

impl ClusterClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(600))
            .pool_max_idle_per_host(5)
            .build()
            .unwrap_or_default();

        let nodes = vec![
            NodeInfo {
                name: "ZimaBoard-1".to_string(),
                ollama_url: "http://192.168.0.206:11434".to_string(),
                zev_url: "http://192.168.0.206:7860".to_string(),
            },
            NodeInfo {
                name: "ZimaBoard-2".to_string(),
                ollama_url: "http://192.168.0.224:11434".to_string(),
                zev_url: "http://192.168.0.224:7860".to_string(),
            },
        ];

        Self { client, nodes }
    }

    pub async fn check_health(&self) -> Vec<(String, bool)> {
        let mut results = Vec::new();
        for node in &self.nodes {
            let url = format!("{}/api/tags", node.ollama_url);
            let ok = match self.client.get(&url).timeout(Duration::from_secs(2)).send().await {
                Ok(resp) => resp.status().is_success(),
                Err(_) => false,
            };
            results.push((node.name.clone(), ok));
        }
        results
    }

    pub async fn query_node(
        &self,
        node: &NodeInfo,
        prompt: &str,
        model: &str,
        max_tokens: u32,
    ) -> Result<(String, f64, f64)> {
        let url = format!("{}/api/generate", node.ollama_url);
        let req_body = OllamaRequest {
            model,
            prompt,
            stream: false,
            options: OllamaOptions {
                num_predict: max_tokens,
                temperature: 0.2,
            },
        };

        let t0 = Instant::now();
        let resp = self
            .client
            .post(&url)
            .json(&req_body)
            .send()
            .await
            .with_context(|| format!("Failed to connect to {}", node.name))?;

        let body: OllamaResponse = resp
            .json()
            .await
            .with_context(|| format!("Failed to parse response from {}", node.name))?;

        let elapsed = t0.elapsed().as_secs_f64();
        let eval_count = body.eval_count.unwrap_or(0);
        let tok_s = if elapsed > 0.0 {
            eval_count as f64 / elapsed
        } else {
            0.0
        };

        let raw_text = body.response.unwrap_or_default();
        let rust_code = extract_rust_fence(&raw_text);

        Ok((rust_code, elapsed, tok_s))
    }
}

pub fn extract_rust_fence(text: &str) -> String {
    if let Some(start) = text.find("```rust") {
        let after = &text[start + 7..];
        if let Some(end) = after.find("```") {
            return after[..end].trim().to_string();
        }
    } else if let Some(start) = text.find("```") {
        let after = &text[start + 3..];
        if let Some(end) = after.find("```") {
            return after[..end].trim().to_string();
        }
    }
    text.trim().to_string()
}
