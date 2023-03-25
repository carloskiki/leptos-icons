#[cfg(feature = "RiLogosLineStackOverflow")]
use leptos::*;
#[cfg(feature = "RiLogosLineStackOverflow")]
///This icon requires the feature `RiLogosLineStackOverflow` to be enabled.
#[component]
pub fn StackOverflow(
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
        "M18 20.002V15h2v7.002H4V15h2v5.002h12zM7.5 18v-2h9v2h-9zm.077-4.38l.347-1.97 8.864 1.563-.348 1.97-8.863-1.563zm1.634-5.504l1-1.732 7.794 4.5-1 1.732-7.794-4.5zm3.417-4.613l1.532-1.286 5.785 6.895-1.532 1.285-5.785-6.894z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
