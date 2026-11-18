//! VEGA: first-run AI setup (P07). Pick a bring-your-own-key provider, add its
//! key (each provider stores it in the OS keychain, exactly as Settings > AI >
//! LLM Providers does) or detect a local server, then choose the default model.

use std::sync::Arc;

use collections::HashMap;
use fs::Fs;
use gpui::{AnyView, App, Context, Empty, Entity, IntoElement, Render, Subscription, Window};
use language_model::{
    IconOrSvg, LanguageModelProvider, LanguageModelProviderId, LanguageModelRegistry,
    ProviderSettingsView,
};
use settings::{LanguageModelSelection, update_settings_file};
use ui::{ButtonLink, ContextMenu, DropdownMenu, prelude::*};
use ui_input::InputField;

/// Providers offered on the welcome page, in order. Every other provider is
/// still available in Settings > AI > LLM Providers.
pub(crate) const FEATURED_PROVIDERS: [&str; 6] = [
    "anthropic",
    "openai",
    "google",
    "openrouter",
    "ollama",
    "lmstudio",
];

pub(crate) struct AiProviderSetup {
    fs: Arc<dyn Fs>,
    selected: Option<LanguageModelProviderId>,
    key_input: Option<Entity<InputField>>,
    configuration_views: HashMap<LanguageModelProviderId, AnyView>,
    _registry_subscription: Subscription,
}

impl AiProviderSetup {
    pub(crate) fn new(fs: Arc<dyn Fs>, cx: &mut Context<Self>) -> Self {
        let registry = LanguageModelRegistry::global(cx);
        let selected = first_configured(&FEATURED_PROVIDERS, |id| {
            registry
                .read(cx)
                .provider(&LanguageModelProviderId::from(id.to_string()))
                .is_some_and(|provider| provider.is_authenticated(cx))
        })
        .map(|id| LanguageModelProviderId::from(id.to_string()));
        Self {
            fs,
            selected,
            key_input: None,
            configuration_views: HashMap::default(),
            _registry_subscription: cx.subscribe(&registry, |_, _, _, cx| cx.notify()),
        }
    }

    fn featured_providers(cx: &App) -> Vec<Arc<dyn LanguageModelProvider>> {
        let registry = LanguageModelRegistry::read_global(cx);
        FEATURED_PROVIDERS
            .iter()
            .filter_map(|id| registry.provider(&LanguageModelProviderId::from(id.to_string())))
            .collect()
    }

    fn select(&mut self, id: LanguageModelProviderId, window: &mut Window, cx: &mut Context<Self>) {
        self.selected = Some(id);
        if let Some(input) = &self.key_input {
            input.update(cx, |input, cx| input.clear(window, cx));
        }
        cx.notify();
    }

    fn save_key(
        &mut self,
        provider: Arc<dyn LanguageModelProvider>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(input) = self.key_input.clone() else {
            return;
        };
        let key = input.read(cx).text(cx).trim().to_string();
        if key.is_empty() {
            return;
        }
        provider.set_api_key(Some(key), cx).detach_and_log_err(cx);
        input.update(cx, |input, cx| input.clear(window, cx));
    }

    fn key_input(&mut self, window: &mut Window, cx: &mut Context<Self>) -> Entity<InputField> {
        self.key_input
            .get_or_insert_with(|| {
                cx.new(|cx| {
                    InputField::new(window, cx, "Paste your API key")
                        .label("API key")
                        .masked(true)
                })
            })
            .clone()
    }

    fn configuration_view(
        &mut self,
        id: &LanguageModelProviderId,
        create: &language_model::CreateProviderSettingsView,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyView {
        self.configuration_views
            .entry(id.clone())
            .or_insert_with(|| create(window, cx))
            .clone()
    }

    fn render_provider_buttons(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let providers = Self::featured_providers(cx);
        h_flex()
            .flex_wrap()
            .gap_2()
            .children(providers.into_iter().map(|provider| {
                let id = provider.id();
                let configured = provider.is_authenticated(cx);
                let is_selected = self.selected.as_ref() == Some(&id);
                let icon = match provider.icon() {
                    IconOrSvg::Svg(path) => Icon::from_external_svg(path),
                    IconOrSvg::Icon(name) => Icon::new(name),
                };
                Button::new(
                    SharedString::from(format!("ai-provider-{}", id.0)),
                    provider.name().0,
                )
                .style(ButtonStyle::Outlined)
                .toggle_state(is_selected)
                .start_icon(icon.size(IconSize::Small).color(Color::Muted))
                .when(configured, |button| {
                    button.end_icon(
                        Icon::new(IconName::Check)
                            .size(IconSize::Small)
                            .color(Color::Success),
                    )
                })
                .on_click(
                    cx.listener(move |this, _, window, cx| this.select(id.clone(), window, cx)),
                )
            }))
    }

    fn render_provider_body(
        &mut self,
        provider: Arc<dyn LanguageModelProvider>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let id = provider.id();
        match provider.settings_view(cx) {
            Some(ProviderSettingsView::ApiKey(config)) if config.has_key => h_flex()
                .gap_2()
                .child(
                    Icon::new(IconName::Check)
                        .size(IconSize::Small)
                        .color(Color::Success),
                )
                .child(Label::new(if config.is_from_env_var {
                    format!("Using the key from {}", config.env_var_name)
                } else {
                    "Key saved in your keychain".to_string()
                }))
                .when(!config.is_from_env_var, |row| {
                    let provider = provider.clone();
                    row.child(
                        Button::new("remove-ai-key", "Remove Key")
                            .style(ButtonStyle::Subtle)
                            .on_click(move |_, _, cx| {
                                provider.set_api_key(None, cx).detach_and_log_err(cx);
                            }),
                    )
                })
                .into_any_element(),
            Some(ProviderSettingsView::ApiKey(config)) => {
                let input = self.key_input(window, cx);
                v_flex()
                    .gap_2()
                    .child(
                        h_flex()
                            .gap_2()
                            .items_end()
                            .child(div().flex_1().child(input))
                            .child(
                                Button::new("save-ai-key", "Save Key")
                                    .style(ButtonStyle::Filled)
                                    .on_click(cx.listener({
                                        let provider = provider.clone();
                                        move |this, _, window, cx| {
                                            this.save_key(provider.clone(), window, cx)
                                        }
                                    })),
                            ),
                    )
                    .child(
                        h_flex()
                            .gap_1()
                            .child(
                                Label::new("Get a key from the")
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            )
                            .child(
                                ButtonLink::new(
                                    format!("{} dashboard", provider.name().0),
                                    config.api_key_url,
                                )
                                .no_icon(true)
                                .label_size(LabelSize::Small)
                                .label_color(Color::Muted),
                            )
                            .child(
                                Label::new("— it's stored in your OS keychain.")
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            ),
                    )
                    .into_any_element()
            }
            Some(ProviderSettingsView::Inline(settings)) => self
                .configuration_view(&id, &settings.create_view, window, cx)
                .into_any_element(),
            Some(ProviderSettingsView::SubPage(settings)) => self
                .configuration_view(&id, &settings.create_view, window, cx)
                .into_any_element(),
            None => Empty.into_any_element(),
        }
    }

    fn render_model_picker(
        &self,
        provider: &Arc<dyn LanguageModelProvider>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<impl IntoElement> {
        if !provider.is_authenticated(cx) {
            return None;
        }
        let models = provider.provided_models(cx);
        if models.is_empty() {
            return None;
        }
        let current = LanguageModelRegistry::read_global(cx)
            .default_model()
            .filter(|model| model.provider_id == provider.id());
        let label = current
            .as_ref()
            .map_or_else(|| "Choose a model".into(), |model| model.name.0.clone());

        let fs = self.fs.clone();
        let menu = ContextMenu::build(window, cx, move |mut menu, _, _| {
            for model in models {
                let fs = fs.clone();
                let name = model.name.0.clone();
                menu = menu.entry(name, None, move |_, cx| {
                    let selection = model_selection(&model.provider_id, &model.id.0);
                    update_settings_file(fs.clone(), cx, move |settings, _| {
                        settings.agent.get_or_insert_default().set_model(selection);
                    });
                });
            }
            menu
        });

        Some(
            h_flex()
                .gap_2()
                .child(Label::new("Default model").color(Color::Muted))
                .child(DropdownMenu::new("ai-default-model", label, menu)),
        )
    }
}

impl Render for AiProviderSetup {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let selected = self
            .selected
            .as_ref()
            .and_then(|id| LanguageModelRegistry::read_global(cx).provider(id));

        v_flex()
            .gap_0p5()
            .child(Label::new("AI Provider"))
            .child(
                Label::new(
                    "Bring your own key, or use a model running on your machine. \
                     You can add more providers later in Settings.",
                )
                .color(Color::Muted),
            )
            .child(div().mt_1p5().child(self.render_provider_buttons(cx)))
            .when_some(selected, |section, provider| {
                let body = self.render_provider_body(provider.clone(), window, cx);
                let picker = self.render_model_picker(&provider, window, cx);
                section.child(v_flex().mt_3().gap_3().child(body).children(picker))
            })
    }
}

/// The first of `featured` that is already configured, so returning users see
/// the provider they use.
fn first_configured<'a>(
    featured: &[&'a str],
    is_configured: impl Fn(&str) -> bool,
) -> Option<&'a str> {
    featured.iter().copied().find(|id| is_configured(id))
}

fn model_selection(provider: &LanguageModelProviderId, model: &str) -> LanguageModelSelection {
    LanguageModelSelection {
        provider: provider.0.to_string().into(),
        model: model.to_string(),
        enable_thinking: false,
        effort: None,
        speed: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preselects_the_first_configured_provider() {
        assert_eq!(
            first_configured(&FEATURED_PROVIDERS, |id| id == "ollama" || id == "openai"),
            Some("openai")
        );
        assert_eq!(first_configured(&FEATURED_PROVIDERS, |_| false), None);
    }

    #[test]
    fn featured_providers_never_include_zed_cloud() {
        assert!(!FEATURED_PROVIDERS.contains(&"zed.dev"));
    }

    #[test]
    fn model_selection_names_provider_and_model() {
        let selection = model_selection(
            &LanguageModelProviderId::from("openai".to_string()),
            "gpt-5",
        );
        assert_eq!(selection.provider.0, "openai");
        assert_eq!(selection.model, "gpt-5");
        assert!(!selection.enable_thinking);
    }
}
