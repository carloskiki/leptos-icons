#[cfg(feature = "VsReport")]
use leptos::*;
#[cfg(feature = "VsReport")]
///This icon requires the feature `VsReport` to be enabled.
#[component]
pub fn Report(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M1.5 1h13l.5.5v10l-.5.5H7.707l-2.853 2.854L4 14.5V12H1.5l-.5-.5v-10l.5-.5zm6 10H14V2H2v9h2.5l.5.5v1.793l2.146-2.147L7.5 11zm0-8h1v5h-1V3zm0 7h1V9h-1v1z"
        /> < title > { title } < / title > < / svg >
    }
}
