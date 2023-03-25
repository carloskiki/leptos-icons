#[cfg(feature = "CgMinimizeAlt")]
use leptos::*;
#[cfg(feature = "CgMinimizeAlt")]
///This icon requires the feature `CgMinimizeAlt` to be enabled.
#[component]
pub fn MinimizeAlt(
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
        "M20.0735 2L21.4877 3.41421L15.6378 9.26416H18.1824V11.2642H12.1824V5.26416H14.1824V7.89111L20.0735 2Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M11.1824 12.2642V18.2642H9.1824V15.8422L3.41421 21.6104L2 20.1962L7.93203 14.2642H5.1824V12.2642H11.1824Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
