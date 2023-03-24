#[cfg(feature = "RiBusinessFillDonutChart")]
use leptos::*;
#[cfg(feature = "RiBusinessFillDonutChart")]
///This icon requires the feature `RiBusinessFillDonutChart` to be enabled.
#[component]
pub fn DonutChart(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = { size.clone() }
        height = { size } > < g xmlns = "http://www.w3.org/2000/svg" >< path fill =
        "none" d = "M0 0H24V24H0z" />< path d =
        "M11 2.05v3.02C7.608 5.557 5 8.475 5 12c0 3.866 3.134 7 7 7 1.572 0 3.024-.518 4.192-1.394l2.137 2.137C16.605 21.153 14.4 22 12 22 6.477 22 2 17.523 2 12c0-5.185 3.947-9.449 9-9.95zM21.95 13c-.2 2.011-.994 3.847-2.207 5.328l-2.137-2.136c.687-.916 1.153-2.006 1.323-3.192h3.022zM13.002 2.05c4.724.469 8.48 4.226 8.95 8.95h-3.022c-.438-3.065-2.863-5.49-5.928-5.929V2.049z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
