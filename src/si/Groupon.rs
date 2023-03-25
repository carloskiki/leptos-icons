#[cfg(feature = "SiGroupon")]
use leptos::*;
#[cfg(feature = "SiGroupon")]
///This icon requires the feature `SiGroupon` to be enabled.
#[component]
pub fn Groupon(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M3.316 20.334C5.618 22.736 8.554 24 12.012 24c3.988 0 7.739-1.95 9.978-5.163 1.353-1.95 2.01-4.158 2.01-6.755 0-.484-.032-1.006-.063-1.529H10.595v4.61h6.687c-1.155 2.012-3.094 3.12-5.27 3.12-3.229 0-6.125-2.824-6.125-6.497 0-3.315 2.699-6.069 6.125-6.069 1.844 0 3.355.749 4.811 2.239h6.52C21.468 3.019 17.084 0 12.083 0c-3.323 0-6.22 1.17-8.53 3.409C1.25 5.647 0 8.572 0 11.754c-.008 3.417 1.108 6.271 3.316 8.58z"
        /> < title > { title } < / title > < / svg >
    }
}
