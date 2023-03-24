#[cfg(feature = "CgCornerDoubleUpRight")]
use leptos::*;
#[cfg(feature = "CgCornerDoubleUpRight")]
///This icon requires the feature `CgCornerDoubleUpRight` to be enabled.
#[component]
pub fn CornerDoubleUpRight(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M14.7495 7.78369L16.1638 6.36948L20.4064 10.6121L16.1637 14.8547L14.7495 13.4405L17.5779 10.6121L14.7495 7.78369Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10.5068 13.4405L11.9211 14.8547L16.1637 10.6121L11.9211 6.36948L10.5069 7.78369L12.3537 9.63049L7.59366 9.63049C5.38452 9.63049 3.59366 11.4214 3.59366 13.6305L3.59366 17.6305L5.59366 17.6305L5.59366 13.6305C5.59366 12.5259 6.48909 11.6305 7.59366 11.6305L12.3169 11.6305L10.5068 13.4405Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
