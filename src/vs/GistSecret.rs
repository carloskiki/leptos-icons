#[cfg(feature = "VsGistSecret")]
use leptos::*;
#[cfg(feature = "VsGistSecret")]
///This icon requires the feature `VsGistSecret` to be enabled.
#[component]
pub fn GistSecret(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M3 14h4v.91l.09.09H2.5l-.5-.5v-13l.5-.5h7.72l.35.14 3.28 3.3.15.36v2.54a3.1 3.1 0 0 0-1-.94V6H9.5L9 5.5V2H3v12zm10-9l-3-3v3h3zm.5 4v1h1l.5.5v4l-.5.5h-6l-.5-.5v-4l.5-.5h1V9a2 2 0 0 1 4 0zm-2.707-.707A1 1 0 0 0 10.5 9v1h2V9a1 1 0 0 0-1.707-.707zM9 11v3h5v-3H9z"
        /> < title > { title } < / title > < / svg >
    }
}
