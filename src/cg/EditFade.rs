#[cfg(feature = "CgEditFade")]
use leptos::*;
#[cfg(feature = "CgEditFade")]
///This icon requires the feature `CgEditFade` to be enabled.
#[component]
pub fn EditFade(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 12C8 10.5194 8.8044 9.22675 10 8.53513V15.4649C8.8044 14.7733 8 13.4806 8 12Z"
        stroke = "currentColor" stroke - opacity = "0.3" stroke - width = "4" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M14 15.4649V8.53513C15.1956 9.22675 16 10.5194 16 12C16 13.4806 15.1956 14.7733 14 15.4649Z"
        stroke = "currentColor" stroke - opacity = "0.6" stroke - width = "4" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M3 12C3 7.02944 7.02944 3 12 3C16.9706 3 21 7.02944 21 12C21 16.9706 16.9706 21 12 21C7.02944 21 3 16.9706 3 12Z"
        stroke = "currentColor" stroke - width = "2" /> < title > { title } < / title > <
        / svg >
    }
}
