#[cfg(feature = "FaSolidDiagramNext")]
use leptos::*;
#[cfg(feature = "FaSolidDiagramNext")]
///This icon requires the feature `FaSolidDiagramNext` to be enabled.
#[component]
pub fn DiagramNext(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M512 160c0 35.3-28.7 64-64 64H280v64h46.1c21.4 0 32.1 25.9 17 41L273 399c-9.4 9.4-24.6 9.4-33.9 0L169 329c-15.1-15.1-4.4-41 17-41H232V224H64c-35.3 0-64-28.7-64-64V96C0 60.7 28.7 32 64 32H448c35.3 0 64 28.7 64 64v64zM448 416V352H365.3l.4-.4c18.4-18.4 20.4-43.7 11-63.6l71.3 0c35.3 0 64 28.7 64 64v64c0 35.3-28.7 64-64 64L64 480c-35.3 0-64-28.7-64-64V352c0-35.3 28.7-64 64-64l71.3 0c-9.4 19.9-7.4 45.2 11 63.6l.4 .4H64v64H210.7l5.7 5.7c21.9 21.9 57.3 21.9 79.2 0l5.7-5.7H448z"
        /> < title > { title } < / title > < / svg >
    }
}
