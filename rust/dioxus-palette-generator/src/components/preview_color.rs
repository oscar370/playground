use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum Status {
    Idle,
    Copied,
    Error,
}

#[derive(PartialEq, Clone, Props)]
pub struct PreviewColorProps {
    color: String,
    name: String,
}

#[component]
pub fn PreviewColor(props: PreviewColorProps) -> Element {
    let mut status = use_signal(|| Status::Idle);
    let color = props.color.clone();

    let tip = match status() {
        Status::Idle => format!("{}: {}", props.name, props.color),
        Status::Copied => "Copied!".to_string(),
        Status::Error => "Fail to copy".to_string(),
    };

    let class = match status() {
        Status::Idle => "tooltip",
        Status::Copied => "tooltip tooltip-success",
        Status::Error => "tooltip tooltip-error",
    };

    rsx! {
        div { class, "data-tip": "{tip}",
            button {
                class: "size-12 rounded-sm border border-(--color-neutral)",
                background_color: "{props.color}",
                onmouseleave: move |_| status.set(Status::Idle),
                onclick: move |_| {
                    let color = color.clone();
                    async move {
                        match handle_copy_color(&color).await {
                            Ok(_) => status.set(Status::Copied),
                            Err(_) => status.set(Status::Error),
                        }
                    }
                },
            }
        }
    }
}

async fn handle_copy_color(color: &str) -> Result<(), document::EvalError> {
    let e = document::eval(
        r#"
        const t = await dioxus.recv();
        await navigator.clipboard.writeText(t);
        return true;
    "#,
    );
    e.send(color)?;
    e.await?;
    Ok(())
}
