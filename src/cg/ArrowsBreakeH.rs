#[cfg(feature = "CgArrowsBreakeH")]
use leptos::*;
#[cfg(feature = "CgArrowsBreakeH")]
///This icon requires the feature `CgArrowsBreakeH` to be enabled.
#[component]
pub fn ArrowsBreakeH(
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
        "M9.24268 7H11.2427V11.0001H11.2477V13.0001H11.2427V17H9.24268V13.0001L4.82846 13L6.65685 14.8284L5.24264 16.2426L1 12L5.24264 7.75732L6.65685 9.17154L4.82839 11H9.24264L9.24268 7Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15.2527 7.00012H13.2527V11.0001H13.2477V13.0001H13.2527V17.0001H15.2527V13.0001L19.667 13L17.8385 14.8285L19.2527 16.2427L23.4954 12L19.2527 7.75739L17.8385 9.17161L19.6669 11H15.2527L15.2527 7.00012Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
