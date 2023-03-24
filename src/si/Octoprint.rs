#[cfg(feature = "SiOctoprint")]
use leptos::*;
#[cfg(feature = "SiOctoprint")]
///This icon requires the feature `SiOctoprint` to be enabled.
#[component]
pub fn Octoprint(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    color: String,
    /// HTML style attribute.
    #[prop(into)]
    #[prop(optional)]
    style: String,
    /// Accessibility title.
    #[prop(into)]
    #[prop(optional)]
    title: String,
) -> impl IntoView {
    let style = format!("{} color: {};", style, color);
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M3.942 4.613C2.424 5.987 1.107 7.473.476 9.71c-.634 2.248-.585 5.094-.145 7.398.44 2.303 1.12 4.107 1.873 5.83h13.179c-.31-.988-.761-1.967-1.446-3.237-.685-1.268-1.658-2.692-2.648-4.178-.99-1.486-1.985-3.077-1.851-4.472.094-.987.49-1.976 1.492-2.76 1.16-.909 2.289-1.437 3.353-1.595 3.325-.496 6.422.601 8.925 3.366.288.316.36.726.545 1.127l.166-.653c.15-.589.088-1.359-.152-2.371-.243-1.029-.563-1.792-1.46-2.973-.893-1.176-2.467-2.322-4.48-3.226-1.5-.673-3.305-1-5.798-.879-2.522.124-5.494 1.177-8.087 3.526Z"
        /> < title > { title } < / title > < / svg >
    }
}
