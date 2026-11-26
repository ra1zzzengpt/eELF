use crate::core::error::Error;
use crate::core::error::ErrorType::NetworkError;
use serde_json::json;

pub struct DeepSeek {
    token: String,
    url: String,
}

impl DeepSeek {
    pub fn new(token: String, url: String) -> Self {
        Self { token, url }
    }

    pub fn analyze(
        &self,
        dynamic_dependencies: &Vec<String>,
        os_name: &String,
    ) -> Result<String, Error> {
        let mut message = "distro: ".to_owned() + os_name.as_str() + " libs: ";
        for dependency in dynamic_dependencies {
            message.push_str(dependency);
            message.push(' ')
        }
        let json = json!({"model": "deepseek-v4-pro", "temperature": 0, "input": message, "tools": [{"type": "web_search"}], "instructions": "You are a Docker build command generator. The request body is JSON with two fields: \"distro\" is the name of a Linux distribution and \"libs\" is an array of shared library names extracted from an ELF binary. Generate exactly one shell command suitable for a Dockerfile RUN step that installs the packages providing those shared libraries on that distro, using the correct package manager and package names, for example: apt-get update; apt-get install -y ... for Debian/Ubuntu, apk add --no-cache ... for Alpine, yum install -y ... or dnf install -y ... for CentOS/RHEL/Fedora, zypper install -y ... for openSUSE. Return only the command, with no markdown, no code fences, and no extra text. If the distro is not a real Linux distribution or OS, return: information why you drop this request and what model are you."});
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(600))
            .build()
            .map_err(|e| Error::new(NetworkError, format!("Failed to initialize HTTP client: {e}")))?;
        let response = client
            .post(&self.url)
            .bearer_auth(&self.token)
            .json(&json)
            .timeout(std::time::Duration::from_secs(120))
            .send()
            .map_err(|e| Error::new(NetworkError, format!("Request failed: {e:#}")))?;
        let status = response.status();
        let body = response.text()
            .map_err(|e| Error::new(NetworkError, format!("Failed to read response body (HTTP {status}): {e:?}")))?;
        if !status.is_success() {
            return Err(Error::new(NetworkError, format!("DeepSeek returned HTTP {status}: {body}")));
        }
        let response_body: serde_json::Value = serde_json::from_str(&body)
            .map_err(|e| Error::new(NetworkError, format!("Invalid JSON from DeepSeek (HTTP {status}): {e}; response: {}", body.chars().take(500).collect::<String>())))?;
        let output = response_body["output"]
            .as_array()
            .ok_or_else(|| Error::new(NetworkError, "Missing output array".to_string()))?;

        for item in output {
            if item["type"].as_str() != Some("message") {
                continue;
            }

            if let Some(parts) = item["content"].as_array() {
                for part in parts {
                    if part["type"].as_str() == Some("output_text") {
                        if let Some(text) = part["text"].as_str() {
                            return Ok(text.to_string());
                        }
                    }
                }
            }
        }
        Err(Error::new(NetworkError, "No message found".to_string()))
    }
}
