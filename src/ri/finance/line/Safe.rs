#[cfg(feature = "RiFinanceLineSafe")]
use leptos::*;
#[cfg(feature = "RiFinanceLineSafe")]
///This icon requires the feature `RiFinanceLineSafe` to be enabled.
#[component]
pub fn Safe(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M18 20H6v2H4v-2H3a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1h18a1 1 0 0 1 1 1v15a1 1 0 0 1-1 1h-1v2h-2v-2zM4 18h16V5H4v13zm9-4.126V17h-2v-3.126A4.002 4.002 0 0 1 12 6a4 4 0 0 1 1 7.874zM12 12a2 2 0 1 0 0-4 2 2 0 0 0 0 4z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
