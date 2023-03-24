#[cfg(feature = "FaSolidBuildingNgo")]
use leptos::*;
#[cfg(feature = "FaSolidBuildingNgo")]
///This icon requires the feature `FaSolidBuildingNgo` to be enabled.
#[component]
pub fn BuildingNgo(
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
        stroke_witdh = "0" style = style viewBox = "0 0 384 512" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M48 0C21.5 0 0 21.5 0 48V464c0 26.5 21.5 48 48 48h96V432c0-26.5 21.5-48 48-48s48 21.5 48 48v80h96c26.5 0 48-21.5 48-48V48c0-26.5-21.5-48-48-48H48zM64 240c0-8.8 7.2-16 16-16h32c8.8 0 16 7.2 16 16v32c0 8.8-7.2 16-16 16H80c-8.8 0-16-7.2-16-16V240zm112-16h32c8.8 0 16 7.2 16 16v32c0 8.8-7.2 16-16 16H176c-8.8 0-16-7.2-16-16V240c0-8.8 7.2-16 16-16zm80 16c0-8.8 7.2-16 16-16h32c8.8 0 16 7.2 16 16v32c0 8.8-7.2 16-16 16H272c-8.8 0-16-7.2-16-16V240zM168 64h48c8.8 0 16 7.2 16 16s-7.2 16-16 16H184v64h16V144c0-8.8 7.2-16 16-16s16 7.2 16 16v24c0 13.3-10.7 24-24 24H176c-13.3 0-24-10.7-24-24V80c0-8.8 7.2-16 16-16zM304 96c-8.8 0-16 7.2-16 16v32c0 8.8 7.2 16 16 16s16-7.2 16-16V112c0-8.8-7.2-16-16-16zm-48 16c0-26.5 21.5-48 48-48s48 21.5 48 48v32c0 26.5-21.5 48-48 48s-48-21.5-48-48V112zM61.3 71.1l34.7 52V80c0-8.8 7.2-16 16-16s16 7.2 16 16v96c0 7.1-4.6 13.3-11.4 15.3s-14-.6-17.9-6.4L64 132.8V176c0 8.8-7.2 16-16 16s-16-7.2-16-16V80c0-7.1 4.6-13.3 11.4-15.3s14 .6 17.9 6.4z"
        /> < title > { title } < / title > < / svg >
    }
}
