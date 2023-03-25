#[cfg(feature = "BiSolidBandAid")]
use leptos::*;
#[cfg(feature = "BiSolidBandAid")]
///This icon requires the feature `BiSolidBandAid` to be enabled.
#[component]
pub fn BandAid(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "m11.646 3.868-7.778 7.778a6.007 6.007 0 0 0 0 8.485 5.984 5.984 0 0 0 4.242 1.754 5.984 5.984 0 0 0 4.243-1.754l7.778-7.778a6.007 6.007 0 0 0 0-8.485 6.008 6.008 0 0 0-8.485 0zM9 13a1 1 0 1 1 0-2 1 1 0 0 1 0 2zm3 3a1 1 0 1 1 0-2 1 1 0 0 1 0 2zm0-6a1 1 0 1 1 0-2 1 1 0 0 1 0 2zm3 3a1 1 0 1 1 0-2 1 1 0 0 1 0 2z"
        /> < title > { title } < / title > < / svg >
    }
}
