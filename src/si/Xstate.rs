#[cfg(feature = "SiXstate")]
use leptos::*;
#[cfg(feature = "SiXstate")]
///This icon requires the feature `SiXstate` to be enabled.
#[component]
pub fn Xstate(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M15.891 0h6.023l-6.085 10.563c-1.853-3.305-1.822-7.32.062-10.563zm6.055 23.999L8.078.001H2.055l6.919 12.015L2.055 24h6.023L12 17.236 15.892 24z"
        /> < title > { title } < / title > < / svg >
    }
}
