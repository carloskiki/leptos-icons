#[cfg(feature = "RiLogosFillFinder")]
use leptos::*;
#[cfg(feature = "RiLogosFillFinder")]
///This icon requires the feature `RiLogosFillFinder` to be enabled.
#[component]
pub fn Finder(
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
        "M21 3c.552 0 1 .448 1 1v16c0 .552-.448 1-1 1H3c-.552 0-1-.448-1-1V4c0-.552.448-1 1-1h18zm-1 2h-8.465c-.69 1.977-1.035 4.644-1.035 8h3c-.115.92-.15 1.878-.107 2.877 1.226-.211 2.704-.777 4.027-1.71l1.135 1.665c-1.642 1.095-3.303 1.779-4.976 2.043.052.37.113.745.184 1.125H20V5zM6.555 14.168l-1.11 1.664C7.602 17.27 9.792 18 12 18v-2c-1.792 0-3.602-.603-5.445-1.832zM17 7c.552 0 1 .448 1 1v1c0 .552-.448 1-1 1s-1-.448-1-1V8c0-.552.448-1 1-1zM7 7c-.552 0-1 .452-1 1v1c0 .552.448 1 1 1s1-.45 1-1V8c0-.552-.448-1-1-1z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
