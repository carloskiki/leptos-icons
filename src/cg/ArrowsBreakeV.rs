#[cfg(feature = "CgArrowsBreakeV")]
use leptos::*;
#[cfg(feature = "CgArrowsBreakeV")]
///This icon requires the feature `CgArrowsBreakeV` to be enabled.
#[component]
pub fn ArrowsBreakeV(
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
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M16.2427 5.24264L14.8285 6.65685L13 4.82839L13.0001 9.24268H17.0001V11.2427H7.00012V9.24268H11.0001L11 4.82846L9.17161 6.65685L7.75739 5.24264L12 1L16.2427 5.24264Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M7 15.2527V13.2527H17V15.2527H13.0001L13 19.6669L14.8284 17.8385L16.2426 19.2527L12 23.4954L7.75732 19.2527L9.17154 17.8385L11 19.667V15.2527L7 15.2527Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
