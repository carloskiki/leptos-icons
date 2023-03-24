#[cfg(feature = "BiSolidPieChartAlt")]
use leptos::*;
#[cfg(feature = "BiSolidPieChartAlt")]
///This icon requires the feature `BiSolidPieChartAlt` to be enabled.
#[component]
pub fn PieChartAlt(
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
        width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M19.071 4.929A9.97 9.97 0 0 0 12 2a9.936 9.936 0 0 0-7.071 2.929C3.04 6.818 2 9.33 2 12s1.04 5.182 2.929 7.071C6.818 20.96 9.33 22 12 22s5.182-1.04 7.071-2.929A9.936 9.936 0 0 0 22 12a9.97 9.97 0 0 0-2.929-7.071zm-1.414 12.728C16.146 19.168 14.137 20 12 20s-4.146-.832-5.657-2.343C4.832 16.146 4 14.137 4 12s.832-4.146 2.343-5.657A7.948 7.948 0 0 1 12 4v8h8a7.948 7.948 0 0 1-2.343 5.657z"
        /> < title > { title } < / title > < / svg >
    }
}
