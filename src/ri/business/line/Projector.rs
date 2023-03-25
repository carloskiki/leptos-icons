#[cfg(feature = "RiBusinessLineProjector")]
use leptos::*;
#[cfg(feature = "RiBusinessLineProjector")]
///This icon requires the feature `RiBusinessLineProjector` to be enabled.
#[component]
pub fn Projector(
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
        "M3 3h18a1 1 0 0 1 1 1v16a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1zm8.126 9H4v7h16v-7h-1.126a4.002 4.002 0 0 1-7.748 0zm0-2a4.002 4.002 0 0 1 7.748 0H20V5H4v5h7.126zM15 13a2 2 0 1 0 0-4 2 2 0 0 0 0 4zm-9 2h2v2H6v-2z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
