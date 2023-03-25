#[cfg(feature = "RiMediaLineHeadphone")]
use leptos::*;
#[cfg(feature = "RiMediaLineHeadphone")]
///This icon requires the feature `RiMediaLineHeadphone` to be enabled.
#[component]
pub fn Headphone(
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
        "M12 4a8 8 0 0 0-8 8h3a2 2 0 0 1 2 2v5a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-7C2 6.477 6.477 2 12 2s10 4.477 10 10v7a2 2 0 0 1-2 2h-3a2 2 0 0 1-2-2v-5a2 2 0 0 1 2-2h3a8 8 0 0 0-8-8zM4 14v5h3v-5H4zm13 0v5h3v-5h-3z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
