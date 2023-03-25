#[cfg(feature = "FaSolidEuroSign")]
use leptos::*;
#[cfg(feature = "FaSolidEuroSign")]
///This icon requires the feature `FaSolidEuroSign` to be enabled.
#[component]
pub fn EuroSign(
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
        stroke_witdh = "0" style = style viewBox = "0 0 320 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M48.1 240c-.1 2.7-.1 5.3-.1 8v16c0 2.7 0 5.3 .1 8H32c-17.7 0-32 14.3-32 32s14.3 32 32 32H60.3C89.9 419.9 170 480 264 480h24c17.7 0 32-14.3 32-32s-14.3-32-32-32H264c-57.9 0-108.2-32.4-133.9-80H256c17.7 0 32-14.3 32-32s-14.3-32-32-32H112.2c-.1-2.6-.2-5.3-.2-8V248c0-2.7 .1-5.4 .2-8H256c17.7 0 32-14.3 32-32s-14.3-32-32-32H130.1c25.7-47.6 76-80 133.9-80h24c17.7 0 32-14.3 32-32s-14.3-32-32-32H264C170 32 89.9 92.1 60.3 176H32c-17.7 0-32 14.3-32 32s14.3 32 32 32H48.1z"
        /> < title > { title } < / title > < / svg >
    }
}
