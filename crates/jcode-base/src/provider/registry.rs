use super::*;

/// Centralizes runtime lookup and lifecycle decisions for provider slots that
/// can have more than one concrete runtime behind a single public provider.
///
/// Today the main case is `ActiveProvider::OpenRouter`: real OpenRouter and
/// direct OpenAI-compatible profiles share the OpenAI-compatible wire protocol,
/// but they are distinct runtime identities and must not overwrite each other.
pub(super) struct ProviderRegistry<'a> {
    provider: &'a MultiProvider,
}

impl<'a> ProviderRegistry<'a> {
    pub(super) fn new(provider: &'a MultiProvider) -> Self {
        Self { provider }
    }

    pub(super) fn real_openrouter(&self) -> Option<Arc<dyn Provider>> {
        self.provider
            .openrouter
            .read()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone()
    }

    pub(super) fn compatible_profile(&self, profile_id: &str) -> Option<Arc<dyn Provider>> {
        self.provider
            .openai_compatible_profiles
            .read()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .get(profile_id)
            .cloned()
    }

    pub(super) fn install_compatible_profile(
        &self,
        profile_id: impl Into<String>,
        runtime: Arc<dyn Provider>,
    ) {
        self.provider
            .openai_compatible_profiles
            .write()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .insert(profile_id.into(), runtime);
    }

    pub(super) fn active_compatible_profile_id(&self) -> Option<String> {
        self.provider
            .active_openai_compatible_profile
            .read()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone()
    }

    pub(super) fn set_active_compatible_profile(&self, profile_id: impl Into<String>) {
        *self
            .provider
            .active_openai_compatible_profile
            .write()
            .unwrap_or_else(|poisoned| poisoned.into_inner()) = Some(profile_id.into());
    }

    pub(super) fn clear_active_compatible_profile(&self) {
        *self
            .provider
            .active_openai_compatible_profile
            .write()
            .unwrap_or_else(|poisoned| poisoned.into_inner()) = None;
    }

    pub(super) fn active_compatible_profile(&self) -> Option<Arc<dyn Provider>> {
        let profile_id = self.active_compatible_profile_id()?;
        self.compatible_profile(&profile_id)
    }

    /// Runtime that should execute requests for the public OpenRouter slot.
    pub(super) fn active_openrouter_execution(&self) -> Option<Arc<dyn Provider>> {
        if let Some(active) = self.active_compatible_profile() {
            return Some(active);
        }
        if let Some(real) = self.real_openrouter() {
            return Some(real);
        }
        // Fallback: Check if there is any named provider configured in config.toml or ~/.jcode/providers/*.json
        let config = crate::config::config();
        let candidate_profile = config
            .provider
            .default_provider
            .as_deref()
            .filter(|p| config.providers.contains_key(*p))
            .or_else(|| config.providers.keys().next().map(|s| s.as_str()));
        if let Some(profile_name) = candidate_profile {
            if let Some(profile_cfg) = config.providers.get(profile_name).cloned() {
                if let Ok(runtime) = external::instantiate_openrouter_runtime(
                    external::OpenRouterRuntimeSpec::NamedProfile {
                        name: profile_name.to_string(),
                        config: profile_cfg,
                    },
                ) {
                    self.install_compatible_profile(profile_name.to_string(), Arc::clone(&runtime));
                    self.set_active_compatible_profile(profile_name.to_string());
                    return Some(runtime);
                }
            }
        }
        None
    }
}
