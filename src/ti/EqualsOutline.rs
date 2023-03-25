#[cfg(feature = "TiEqualsOutline")]
use leptos::*;
#[cfg(feature = "TiEqualsOutline")]
///This icon requires the feature `TiEqualsOutline` to be enabled.
#[component]
pub fn EqualsOutline(
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
        "M18 12h-12c-1.654 0-3-1.346-3-3s1.346-3 3-3h12c1.654 0 3 1.346 3 3s-1.346 3-3 3zm-12-4c-.552 0-1 .449-1 1s.448 1 1 1h12c.552 0 1-.449 1-1s-.448-1-1-1h-12zM18 19h-12c-1.654 0-3-1.346-3-3s1.346-3 3-3h12c1.654 0 3 1.346 3 3s-1.346 3-3 3zm-12-4c-.552 0-1 .449-1 1s.448 1 1 1h12c.552 0 1-.449 1-1s-.448-1-1-1h-12z"
        /> < title > { title } < / title > < / svg >
    }
}
