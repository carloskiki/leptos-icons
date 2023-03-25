#[cfg(feature = "TbSquare6Filled")]
use leptos::*;
#[cfg(feature = "TbSquare6Filled")]
///This icon requires the feature `TbSquare6Filled` to be enabled.
#[component]
pub fn Square6Filled(
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
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-square-6-filled" width = "24" height = "24" viewBox
        = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M18 3a3 3 0 0 1 2.995 2.824l.005 .176v12a3 3 0 0 1 -2.824 2.995l-.176 .005h-12a3 3 0 0 1 -2.995 -2.824l-.005 -.176v-12a3 3 0 0 1 2.824 -2.995l.176 -.005h12zm-5 4h-2l-.15 .005a2 2 0 0 0 -1.844 1.838l-.006 .157v6l.005 .15a2 2 0 0 0 1.838 1.844l.157 .006h2l.15 -.005a2 2 0 0 0 1.844 -1.838l.006 -.157v-2l-.005 -.15a2 2 0 0 0 -1.838 -1.844l-.157 -.006h-2v-2h2l.007 .117a1 1 0 0 0 1.993 -.117a2 2 0 0 0 -1.85 -1.995l-.15 -.005zm0 6v2h-2v-2h2z"
        stroke - width = "0" fill = "currentColor" /> < title > { title } < / title > < /
        svg >
    }
}
