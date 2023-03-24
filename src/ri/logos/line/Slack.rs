#[cfg(feature = "RiLogosLineSlack")]
use leptos::*;
#[cfg(feature = "RiLogosLineSlack")]
///This icon requires the feature `RiLogosLineSlack` to be enabled.
#[component]
pub fn Slack(
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
    let style = format!("{} color: {};", style, color);
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M14.5 3A1.5 1.5 0 0 1 16 4.5v5a1.5 1.5 0 0 1-3 0v-5A1.5 1.5 0 0 1 14.5 3zm-10 10H6v1.5A1.5 1.5 0 1 1 4.5 13zm8.5 5h1.5a1.5 1.5 0 1 1-1.5 1.5V18zm1.5-5h5a1.5 1.5 0 0 1 0 3h-5a1.5 1.5 0 0 1 0-3zm5-5a1.5 1.5 0 0 1 0 3H18V9.5A1.5 1.5 0 0 1 19.5 8zm-15 0h5a1.5 1.5 0 0 1 0 3h-5a1.5 1.5 0 0 1 0-3zm5-5A1.5 1.5 0 0 1 11 4.5V6H9.5a1.5 1.5 0 0 1 0-3zm0 10a1.5 1.5 0 0 1 1.5 1.5v5a1.5 1.5 0 0 1-3 0v-5A1.5 1.5 0 0 1 9.5 13z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
