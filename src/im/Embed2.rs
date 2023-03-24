#[cfg(feature = "ImEmbed2")]
use leptos::*;
#[cfg(feature = "ImEmbed2")]
///This icon requires the feature `ImEmbed2` to be enabled.
#[component]
pub fn Embed2(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style version = "1.1" width = "20" height = "16"
        viewBox = "0 0 20 16" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" xmlns : xlink = "http://www.w3.org/1999/xlink" fill
        = "#000000" d = "M13 11.5l1.5 1.5 5-5-5-5-1.5 1.5 3.5 3.5z" />< path xmlns =
        "http://www.w3.org/2000/svg" xmlns : xlink = "http://www.w3.org/1999/xlink" fill
        = "#000000" d = "M7 4.5l-1.5-1.5-5 5 5 5 1.5-1.5-3.5-3.5z" />< path xmlns =
        "http://www.w3.org/2000/svg" xmlns : xlink = "http://www.w3.org/1999/xlink" fill
        = "#000000" d = "M10.958 2.352l1.085 0.296-3 11-1.085-0.296 3-11z" /> < title > {
        title } < / title > < / svg >
    }
}
