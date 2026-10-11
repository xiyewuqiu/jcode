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
    #[serde(default, alias = "base_url", alias = "endpoint", alias = "url")]
    pub api_base: String,
    #[serde(default, alias = "key", alias = "token")]
    pub api_key: Option<String>,
    #[serde(default, alias = "model")]
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
        #[serde(default)]
        context_window: Option<usize>,
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
    if cline_pass_path.exists() {
        return;
    }

    // Try to recover any pre-existing Cline Pass API key from env or config file
    let recovered_key = std::env::var("CLINE_PASS_API_KEY").ok().or_else(|| {
        crate::provider_catalog::load_env_value_from_config_file("CLINE_PASS_API_KEY", "cline-pass.env")
    }).unwrap_or_default();

    let template = serde_json::json!({
        "id": "cline-pass",
        "name": "Cline Pass",
        "api_base": "https://api.cline.bot/api/v1",
        "api_key": recovered_key,
        "default_model": "mistralai/mistral-large-4-0",
        "models": [
            { "id": "mistralai/mistral-large-4-0", "name": "Mistral Large 4.0" },
            { "id": "claude-3-7-sonnet-20250219", "name": "Claude 3.7 Sonnet" },
            { "id": "claude-3-5-sonnet-20241022", "name": "Claude 3.5 Sonnet" },
            { "id": "gpt-4o", "name": "GPT-4o" },
            { "id": "o3-mini", "name": "o3-mini" },
            { "id": "deepseek-chat", "name": "DeepSeek V3" },
            { "id": "deepseek-reasoner", "name": "DeepSeek R1" },
            { "id": "qwen-2.5-coder-32b-instruct", "name": "Qwen 2.5 Coder 32B" }
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
        let mut api_key = provider_def.api_key;

        // Support env var resolution, e.g. "env:MY_API_KEY"
        if let Some(key_val) = &api_key {
            if let Some(env_name) = key_val.strip_prefix("env:") {
                api_key = std::env::var(env_name.trim()).ok();
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
                JsonModelEntry::Detailed { id, name: _, context_window } => {
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
