use crate::{
    components::preview_color::PreviewColor,
    errors::{handle_error, AppError},
    generator::build_theme_with_bytes,
};
use dioxus::prelude::*;
use material_colors::theme::Theme;

#[component]
pub fn Home() -> Element {
    let mut theme = use_signal(|| None::<Theme>);

    async fn load_theme(evt: Event<FormData>) -> Result<Option<Theme>, AppError> {
        let Some(file) = evt.files().first().cloned() else {
            return Ok(None);
        };
        let bytes = file
            .read_bytes()
            .await
            .map_err(|_| AppError::InvalidImage)?;
        let theme = build_theme_with_bytes(bytes)?;
        Ok(Some(theme))
    }

    rsx! {
        main { class: "max-w-150 min-h-svh flex flex-col items-center justify-center mx-auto",
            input {
                class: "file-input",
                r#type: "file",
                onchange: move |evt| async move {
                    match load_theme(evt).await {
                        Ok(Some(new_theme)) => theme.set(Some(new_theme)),
                        Ok(None) => {}
                        Err(err) => handle_error(err),
                    }
                },
            }

            if let Some(current) = theme.read().as_ref() {
                section {
                    h2 {
                        "Dark palette"
                        div { class: "grid grid-cols-6 md:grid-cols-10 gap-x-2 mt-2",
                            for (name , color) in current.schemes.dark.clone() {
                                PreviewColor { color, name }
                            }
                        }
                    }
                }

                section {
                    h2 {
                        "Ligth palette"
                        div { class: "grid grid-cols-6 md:grid-cols-10 gap-x-2 mt-2",
                            for (name , color) in current.schemes.light.clone() {
                                PreviewColor { color, name }
                            }
                        }
                    }
                }
            }
        }

    }
}
