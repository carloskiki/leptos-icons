#[cfg(feature = "RiBusinessFillCloudOff")]
use leptos::*;
#[cfg(feature = "RiBusinessFillCloudOff")]
///This icon requires the feature `RiBusinessFillCloudOff` to be enabled.
#[component]
pub fn CloudOff(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M3.515 2.1l19.092 19.092-1.415 1.415-2.014-2.015A5.985 5.985 0 0 1 17 21H7A6 6 0 0 1 5.008 9.339a6.992 6.992 0 0 1 .353-2.563L2.1 3.514 3.515 2.1zM17 9a6.003 6.003 0 0 1 5.204 8.989L14.01 9.796C14.89 9.29 15.91 9 17 9zm-5-7a7.003 7.003 0 0 1 6.765 5.195 8.027 8.027 0 0 0-6.206 1.15L7.694 3.48A6.97 6.97 0 0 1 12 2z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
