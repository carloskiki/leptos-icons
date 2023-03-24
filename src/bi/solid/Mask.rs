#[cfg(feature = "BiSolidMask")]
use leptos::*;
#[cfg(feature = "BiSolidMask")]
///This icon requires the feature `BiSolidMask` to be enabled.
#[component]
pub fn Mask(
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
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M19 6H5C3.346 6 2 7.346 2 9v5c0 2.206 1.794 4 4 4h1.637c1.166 0 2.28-.557 2.981-1.491.66-.879 2.104-.88 2.764.001A3.744 3.744 0 0 0 16.363 18H18c2.206 0 4-1.794 4-4V9c0-1.654-1.346-3-3-3zM7.5 13C6.119 13 5 12.328 5 11.5S6.119 10 7.5 10s2.5.672 2.5 1.5S8.881 13 7.5 13zm9 0c-1.381 0-2.5-.672-2.5-1.5s1.119-1.5 2.5-1.5 2.5.672 2.5 1.5-1.119 1.5-2.5 1.5z"
        /> < title > { title } < / title > < / svg >
    }
}
