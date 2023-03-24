#[cfg(feature = "CgArrowLongDownE")]
use leptos::*;
#[cfg(feature = "CgArrowLongDownE")]
///This icon requires the feature `CgArrowLongDownE` to be enabled.
#[component]
pub fn ArrowLongDownE(
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
        "M14.998 1.02014H8.99801V7.02014H10.9991L11.012 19.1652L9.16751 17.3309L7.75726 18.7491L12.0118 22.9799L16.2427 18.7253L14.8246 17.315L13.012 19.1378L12.9991 7.02014H14.998V1.02014ZM10.998 3.02014H12.998V5.02014H10.998V3.02014Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
