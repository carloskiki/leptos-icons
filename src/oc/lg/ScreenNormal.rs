#[cfg(feature = "OcLgScreenNormal")]
use leptos::*;
#[cfg(feature = "OcLgScreenNormal")]
///This icon requires the feature `OcLgScreenNormal` to be enabled.
#[component]
pub fn ScreenNormal(
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
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M15.75 3a.75.75 0 0 1 .75.75v3.5c0 .138.112.25.25.25h3.5a.75.75 0 0 1 0 1.5h-3.5A1.75 1.75 0 0 1 15 7.25v-3.5a.75.75 0 0 1 .75-.75Zm-7.5 0a.75.75 0 0 1 .75.75v3.5A1.75 1.75 0 0 1 7.25 9h-3.5a.75.75 0 0 1 0-1.5h3.5a.25.25 0 0 0 .25-.25v-3.5A.75.75 0 0 1 8.25 3ZM3 15.75a.75.75 0 0 1 .75-.75h3.5c.966 0 1.75.784 1.75 1.75v3.5a.75.75 0 0 1-1.5 0v-3.5a.25.25 0 0 0-.25-.25h-3.5a.75.75 0 0 1-.75-.75Zm12 1c0-.966.784-1.75 1.75-1.75h3.5a.75.75 0 0 1 0 1.5h-3.5a.25.25 0 0 0-.25.25v3.5a.75.75 0 0 1-1.5 0Z"
        /> < title > { title } < / title > < / svg >
    }
}
