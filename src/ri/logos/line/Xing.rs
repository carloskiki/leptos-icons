#[cfg(feature = "RiLogosLineXing")]
use leptos::*;
#[cfg(feature = "RiLogosLineXing")]
///This icon requires the feature `RiLogosLineXing` to be enabled.
#[component]
pub fn Xing(
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
        "M20.444 3.5L13.81 14.99 17.857 22h-2.31l-4.045-7.009H11.5L18.134 3.5h2.31zM8.31 7l2.422 4.196-.002.001L7.67 16.5H5.361l3.06-5.305L6.002 7H8.31z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
