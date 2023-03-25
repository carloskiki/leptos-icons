#[cfg(feature = "TiBriefcase")]
use leptos::*;
#[cfg(feature = "TiBriefcase")]
///This icon requires the feature `TiBriefcase` to be enabled.
#[component]
pub fn Briefcase(
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
        = "http://www.w3.org/2000/svg" > < g xmlns = "http://www.w3.org/2000/svg" >< path
        d =
        "M18 7c0-1.654-1.346-3-3-3h-6c-1.654 0-3 1.346-3 3-1.654 0-3 1.346-3 3v7c0 1.654 1.346 3 3 3h12c1.654 0 3-1.346 3-3v-7c0-1.654-1.346-3-3-3zm-9-1h6c.551 0 1 .449 1 1h-8c0-.551.449-1 1-1zm10 11c0 .551-.449 1-1 1h-12c-.551 0-1-.449-1-1v-1h14v1zm-14-2v-5c0-.551.449-1 1-1h12c.551 0 1 .449 1 1v5h-14zM13 12h-2c-.55 0-1 .45-1 1s.45 1 1 1h2c.55 0 1-.45 1-1s-.45-1-1-1z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
