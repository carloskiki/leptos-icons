#[cfg(feature = "TiMediaPause")]
use leptos::*;
#[cfg(feature = "TiMediaPause")]
///This icon requires the feature `TiMediaPause` to be enabled.
#[component]
pub fn MediaPause(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 6c-1.104 0-2 .896-2 2v8c0 1.104.896 2 2 2s2-.896 2-2v-8c0-1.104-.896-2-2-2zM15 6c-1.104 0-2 .896-2 2v8c0 1.104.896 2 2 2s2-.896 2-2v-8c0-1.104-.896-2-2-2z"
        /> < title > { title } < / title > < / svg >
    }
}
