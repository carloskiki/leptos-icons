#[cfg(feature = "RiMediaLineDisc")]
use leptos::*;
#[cfg(feature = "RiMediaLineDisc")]
///This icon requires the feature `RiMediaLineDisc` to be enabled.
#[component]
pub fn Disc(
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
        "M15 4.582V12a3 3 0 1 1-2-2.83V2.05c5.053.501 9 4.765 9 9.95 0 5.523-4.477 10-10 10S2 17.523 2 12c0-5.185 3.947-9.449 9-9.95v2.012A8.001 8.001 0 0 0 12 20a8 8 0 0 0 3-15.418z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
