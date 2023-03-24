#[cfg(feature = "SiRtlzwei")]
use leptos::*;
#[cfg(feature = "SiRtlzwei")]
///This icon requires the feature `SiRtlzwei` to be enabled.
#[component]
pub fn Rtlzwei(
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
        "M0 0v24h7.38v-3.69H3.692L3.69 3.69h9.229V0H0zm16.61 0v3.69h3.7v16.62h-9.238V24H24V0h-7.39zm-.003 6.49l-3.689.717v9.227l3.69-.715V6.49zm-5.535 1.076l-3.69.715v9.229l3.69-.717V7.566z"
        /> < title > { title } < / title > < / svg >
    }
}
