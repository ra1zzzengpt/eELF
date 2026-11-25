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
        let client = reqwest::blocking::Client::new();
        let response = client
            .post(&self.url)
            .bearer_auth(&self.token)
            .json(&json)
            .send()
            .map_err(|e| Error::new(NetworkError, e.to_string()))?
            .error_for_status()
            .map_err(|e| Error::new(NetworkError, e.to_string()))?;
        let response_body: serde_json::Value = response
            .json()
            .map_err(|e| Error::new(NetworkError, e.to_string()))?;
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
