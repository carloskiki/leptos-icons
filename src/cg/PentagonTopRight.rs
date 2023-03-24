#[cfg(feature = "CgPentagonTopRight")]
use leptos::*;
#[cfg(feature = "CgPentagonTopRight")]
///This icon requires the feature `CgPentagonTopRight` to be enabled.
#[component]
pub fn PentagonTopRight(
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
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M10.3301 15.1601L5 14.3922L10 5.73193L15.3301 6.49988L18.6603 10.7319L13.6603 19.3922L10.3301 15.1601ZM13.4052 15.8339L11.4072 13.2947L8.20908 12.8339L11.055 7.90459L14.2531 8.36536L16.2512 10.9046L13.4052 15.8339Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
