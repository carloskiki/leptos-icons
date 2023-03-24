#[cfg(feature = "HiLgOutlinePaperAirplane")]
use leptos::*;
#[cfg(feature = "HiLgOutlinePaperAirplane")]
///This icon requires the feature `HiLgOutlinePaperAirplane` to be enabled.
#[component]
pub fn PaperAirplane(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M5.99972 12.0005L3.2688 3.125C9.88393 5.04665 16.0276 8.07649 21.4855 12.0002C16.0276 15.924 9.884 18.9539 3.26889 20.8757L5.99972 12.0005ZM5.99972 12.0005L13.5 12.0005"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" /> < title > { title } < / title > < / svg >
    }
}
