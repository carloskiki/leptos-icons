#[cfg(feature = "TbPuzzleFilled")]
use leptos::*;
#[cfg(feature = "TbPuzzleFilled")]
///This icon requires the feature `TbPuzzleFilled` to be enabled.
#[component]
pub fn PuzzleFilled(
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
        "icon icon-tabler icon-tabler-puzzle-filled" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10 2a3 3 0 0 1 2.995 2.824l.005 .176v1h3a2 2 0 0 1 1.995 1.85l.005 .15v3h1a3 3 0 0 1 .176 5.995l-.176 .005h-1v3a2 2 0 0 1 -1.85 1.995l-.15 .005h-3a2 2 0 0 1 -1.995 -1.85l-.005 -.15v-1a1 1 0 0 0 -1.993 -.117l-.007 .117v1a2 2 0 0 1 -1.85 1.995l-.15 .005h-3a2 2 0 0 1 -1.995 -1.85l-.005 -.15v-3a2 2 0 0 1 1.85 -1.995l.15 -.005h1a1 1 0 0 0 .117 -1.993l-.117 -.007h-1a2 2 0 0 1 -1.995 -1.85l-.005 -.15v-3a2 2 0 0 1 1.85 -1.995l.15 -.005h3v-1a3 3 0 0 1 3 -3z"
        stroke - width = "0" fill = "currentColor" /> < title > { title } < / title > < /
        svg >
    }
}
