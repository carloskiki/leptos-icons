#[cfg(feature = "CgBowl")]
use leptos::*;
#[cfg(feature = "CgBowl")]
///This icon requires the feature `CgBowl` to be enabled.
#[component]
pub fn Bowl(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M20.5468 3.67162C20.1563 3.28109 19.5231 3.28109 19.1326 3.67162L13.7687 9.03555H2V11.0356H2.00842C2.22563 16.3663 6.61591 20.6213 12 20.6213C17.3841 20.6213 21.7744 16.3663 21.9916 11.0356H22V9.03555H16.5971L20.5468 5.08583C20.9374 4.69531 20.9374 4.06214 20.5468 3.67162ZM14.1762 11.0356C14.1806 11.0356 14.1851 11.0356 14.1896 11.0356H19.9895C19.7739 15.2613 16.2793 18.6213 12 18.6213C7.72066 18.6213 4.22609 15.2613 4.01054 11.0356H14.1762Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
