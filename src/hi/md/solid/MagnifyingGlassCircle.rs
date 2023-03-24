#[cfg(feature = "HiMdSolidMagnifyingGlassCircle")]
use leptos::*;
#[cfg(feature = "HiMdSolidMagnifyingGlassCircle")]
///This icon requires the feature `HiMdSolidMagnifyingGlassCircle` to be enabled.
#[component]
pub fn MagnifyingGlassCircle(
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M6.5 9C6.5 7.61929 7.61929 6.5 9 6.5C10.3807 6.5 11.5 7.61929 11.5 9C11.5 9.69056 11.221 10.3145 10.7678 10.7678C10.3145 11.221 9.69056 11.5 9 11.5C7.61929 11.5 6.5 10.3807 6.5 9Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM9 5C6.79086 5 5 6.79086 5 9C5 11.2091 6.79086 13 9 13C9.8332 13 10.6076 12.7447 11.2481 12.3088L12.7197 13.7803C13.0126 14.0732 13.4874 14.0732 13.7803 13.7803C14.0732 13.4874 14.0732 13.0126 13.7803 12.7197L12.3088 11.2481C12.7447 10.6076 13 9.8332 13 9C13 6.79086 11.2091 5 9 5Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
