use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use jcode_config_types::{
    NamedProviderAuth, NamedProviderConfig, NamedProviderModelConfig, NamedProviderType,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct JsonProviderFile {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default, alias = "base_url", alias = "endpoint", alias = "url", alias = "baseUrl", alias = "baseURL", alias = "apiBase", alias = "api_url", alias = "apiUrl")]
    pub api_base: String,
    #[serde(default, alias = "key", alias = "token", alias = "apiKey", alias = "apikey", alias = "api_key", alias = "api-key", alias = "API_KEY", alias = "APIKEY")]
    pub api_key: Option<String>,
    #[serde(default, alias = "model", alias = "defaultModel")]
    pub default_model: Option<String>,
    #[serde(default)]
    pub models: Vec<JsonModelEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonModelEntry {
    Simple(String),
    Detailed {
        id: String,
        #[serde(default)]
        name: Option<String>,
        #[serde(default, alias = "context_limit", alias = "context-length", alias = "context-window")]
        context_window: Option<usize>,
        #[serde(default, alias = "max_output_tokens", alias = "max-tokens", alias = "max-output-tokens")]
        max_tokens: Option<usize>,
    },
}

pub fn providers_dir() -> anyhow::Result<PathBuf> {
    let dir = crate::storage::jcode_dir()?.join("providers");
    if !dir.exists() {
        let _ = fs::create_dir_all(&dir);
    }
    Ok(dir)
}

/// Ensure a default cline-pass.json exists if the providers folder is empty.
pub fn ensure_default_provider_template(dir: &Path) {
    let cline_pass_path = dir.join("cline-pass.json");

    // Try to recover any pre-existing Cline Pass API key from env or config file
    let recovered_key = std::env::var("CLINE_PASS_API_KEY").ok().or_else(|| {
        crate::provider_catalog::load_env_value_from_config_file("CLINE_PASS_API_KEY", "cline-pass.env")
    }).unwrap_or_default();

    if cline_pass_path.exists() {
        if let Ok(content) = fs::read_to_string(&cline_pass_path) {
            if let Ok(mut parsed) = serde_json::from_str::<serde_json::Value>(&content) {
                let mut modified = false;
                if let Some(def) = parsed.get("default_model").and_then(|v| v.as_str()) {
                    if def == "cline-pass/deepseek-v4.1-flash" || def == "deepseek-v4.1-flash" {
                        parsed["default_model"] = serde_json::Value::String("deepseek/deepseek-v4.1-flash".to_string());
                        modified = true;
                    }
                }
                if let Some(models) = parsed.get_mut("models").and_then(|m| m.as_array_mut()) {
                    for m in models.iter_mut() {
                        if let Some(id) = m.get("id").and_then(|v| v.as_str()) {
                            let new_id = match id {
                                "cline-pass/deepseek-v4.1-flash" | "deepseek-v4.1-flash" => Some("deepseek/deepseek-v4.1-flash"),
                                "claude-3-7-sonnet-20250219" => Some("anthropic/claude-3-7-sonnet-20250219"),
                                "claude-3-5-sonnet-20241022" => Some("anthropic/claude-3-5-sonnet-20241022"),
                                "gpt-4o" => Some("openai/gpt-4o"),
                                "o3-mini" => Some("openai/o3-mini"),
                                "deepseek-chat" => Some("deepseek/deepseek-chat"),
                                "deepseek-reasoner" => Some("deepseek/deepseek-reasoner"),
                                _ => None,
                            };
                            if let Some(nid) = new_id {
                                m["id"] = serde_json::Value::String(nid.to_string());
                                modified = true;
                            }
                        }
                    }
                }
                if modified {
                    if let Ok(formatted) = serde_json::to_string_pretty(&parsed) {
                        let _ = fs::write(&cline_pass_path, formatted);
                    }
                }
            }
        }
        return;
    }

    let template = serde_json::json!({
        "id": "cline-pass",
        "name": "cline-pass",
        "api_base": "https://api.cline.bot/api/v1",
        "api_key": recovered_key,
        "default_model": "deepseek/deepseek-v4.1-flash",
        "models": [
            {
                "id": "deepseek/deepseek-v4.1-flash",
                "name": "DeepSeek V4.1 Flash",
                "context_window": 1000000,
                "max_tokens": 384000
            },
            {
                "id": "anthropic/claude-3-7-sonnet-20250219",
                "name": "Claude 3.7 Sonnet",
                "context_window": 200000,
                "max_tokens": 64000
            },
            {
                "id": "anthropic/claude-3-5-sonnet-20241022",
                "name": "Claude 3.5 Sonnet",
                "context_window": 200000,
                "max_tokens": 8192
            },
            {
                "id": "openai/gpt-4o",
                "name": "GPT-4o",
                "context_window": 128000,
                "max_tokens": 16384
            },
            {
                "id": "openai/o3-mini",
                "name": "o3-mini",
                "context_window": 200000,
                "max_tokens": 100000
            },
            {
                "id": "deepseek/deepseek-chat",
                "name": "DeepSeek V3",
                "context_window": 64000,
                "max_tokens": 8192
            },
            {
                "id": "deepseek/deepseek-reasoner",
                "name": "DeepSeek R1",
                "context_window": 64000,
                "max_tokens": 8192
            }
        ]
    });

    if let Ok(formatted) = serde_json::to_string_pretty(&template) {
        let _ = fs::write(&cline_pass_path, formatted);
    }
}

/// Load all provider definitions from `~/.jcode/providers/*.json`
pub fn load_providers_from_directory() -> Vec<(String, NamedProviderConfig)> {
    let Ok(dir) = providers_dir() else {
        return Vec::new();
    };

    ensure_default_provider_template(&dir);

    let Ok(entries) = fs::read_dir(&dir) else {
        return Vec::new();
    };

    let mut results = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }

        let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };

        let Ok(content) = fs::read_to_string(&path) else {
            continue;
        };

        let provider_def: JsonProviderFile = match serde_json::from_str(&content) {
            Ok(p) => p,
            Err(err) => {
                crate::logging::warn(&format!(
                    "Failed to parse provider config {}: {}",
                    path.display(),
                    err
                ));
                continue;
            }
        };

        if provider_def.api_base.trim().is_empty() {
            continue;
        }

        let provider_id = provider_def.id.unwrap_or_else(|| file_stem.to_string());
        let mut api_key = provider_def.api_key.map(|k| k.trim().to_string()).filter(|k| !k.is_empty());

        // Support env var resolution, e.g. "env:MY_API_KEY"
        if let Some(key_val) = &api_key {
            if let Some(env_name) = key_val.strip_prefix("env:") {
                api_key = std::env::var(env_name.trim()).ok().map(|k| k.trim().to_string()).filter(|k| !k.is_empty());
            }
        }

        let mut models = Vec::new();
        for m in provider_def.models {
            match m {
                JsonModelEntry::Simple(id) => {
                    let id_trimmed = id.trim().to_string();
                    if !id_trimmed.is_empty() {
                        models.push(NamedProviderModelConfig {
                            id: id_trimmed,
                            ..Default::default()
                        });
                    }
                }
                JsonModelEntry::Detailed { id, name: _, context_window, max_tokens: _ } => {
                    let id_trimmed = id.trim().to_string();
                    if !id_trimmed.is_empty() {
                        models.push(NamedProviderModelConfig {
                            id: id_trimmed,
                            context_window,
                            ..Default::default()
                        });
                    }
                }
            }
        }

        let default_model = provider_def.default_model.or_else(|| models.first().map(|m| m.id.clone()));

        let cfg = NamedProviderConfig {
            provider_type: NamedProviderType::OpenAiCompatible,
            base_url: provider_def.api_base.trim().to_string(),
            auth: NamedProviderAuth::Bearer,
            api_key: api_key.filter(|k| !k.trim().is_empty()),
            default_model,
            requires_api_key: Some(true),
            model_catalog: true,
            models,
            ..Default::default()
        };

        results.push((provider_id, cfg));
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_provider_with_various_apikey_aliases() {
        let json_camel = r#"{
            "id": "cline-pass",
            "baseURL": "https://api.cline.bot/api/v1",
            "apiKey": "sk-test-camel",
            "defaultModel": "deepseek-v4.1-flash"
        }"#;
        let parsed: JsonProviderFile = serde_json::from_str(json_camel).unwrap();
        assert_eq!(parsed.api_base, "https://api.cline.bot/api/v1");
        assert_eq!(parsed.api_key.as_deref(), Some("sk-test-camel"));
        assert_eq!(parsed.default_model.as_deref(), Some("deepseek-v4.1-flash"));

        let json_lower = r#"{
            "id": "cline-pass",
            "url": "https://api.cline.bot/api/v1",
            "apikey": "sk-test-lower"
        }"#;
        let parsed2: JsonProviderFile = serde_json::from_str(json_lower).unwrap();
        assert_eq!(parsed2.api_base, "https://api.cline.bot/api/v1");
        assert_eq!(parsed2.api_key.as_deref(), Some("sk-test-lower"));

        let json_upper = r#"{
            "id": "cline-pass",
            "api_base": "https://api.cline.bot/api/v1",
            "API_KEY": "sk-test-upper"
        }"#;
        let parsed3: JsonProviderFile = serde_json::from_str(json_upper).unwrap();
        assert_eq!(parsed3.api_key.as_deref(), Some("sk-test-upper"));
    }

    #[test]
    fn test_ensure_default_provider_template_upgrades_legacy_model_ids() {
        let temp_dir = tempfile::tempdir().unwrap();
        let cline_pass_path = temp_dir.path().join("cline-pass.json");
        let legacy_content = r#"{
            "id": "cline-pass",
            "api_base": "https://api.cline.bot/api/v1",
            "apiKey": "sk-my-secret-key",
            "default_model": "cline-pass/deepseek-v4.1-flash",
            "models": [
                { "id": "cline-pass/deepseek-v4.1-flash", "name": "DeepSeek V4.1 Flash" },
                { "id": "claude-3-7-sonnet-20250219", "name": "Claude 3.7 Sonnet" }
            ]
        }"#;
        std::fs::write(&cline_pass_path, legacy_content).unwrap();

        ensure_default_provider_template(temp_dir.path());

        let upgraded_str = std::fs::read_to_string(&cline_pass_path).unwrap();
        let upgraded: serde_json::Value = serde_json::from_str(&upgraded_str).unwrap();
        assert_eq!(upgraded["default_model"], "deepseek/deepseek-v4.1-flash");
        assert_eq!(upgraded["models"][0]["id"], "deepseek/deepseek-v4.1-flash");
        assert_eq!(upgraded["models"][1]["id"], "anthropic/claude-3-7-sonnet-20250219");
        assert_eq!(upgraded["apiKey"], "sk-my-secret-key");
    }
}


