#[cfg(feature = "HiLgSolidPlayPause")]
use leptos::*;
#[cfg(feature = "HiLgSolidPlayPause")]
///This icon requires the feature `HiLgSolidPlayPause` to be enabled.
#[component]
pub fn PlayPause(
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
        "http://www.w3.org/2000/svg" d =
        "M15 6.75C14.5858 6.75 14.25 7.08579 14.25 7.5V18C14.25 18.1989 14.329 18.3897 14.4697 18.5303C14.6103 18.671 14.8011 18.75 15 18.75H15.75C16.1642 18.75 16.5 18.4142 16.5 18V7.5C16.5 7.08579 16.1642 6.75 15.75 6.75H15Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M20.25 6.75C19.8358 6.75 19.5 7.08579 19.5 7.5V18C19.5 18.4142 19.8358 18.75 20.25 18.75H21C21.4142 18.75 21.75 18.4142 21.75 18L21.75 7.5C21.75 7.30109 21.671 7.11032 21.5303 6.96967C21.3897 6.82902 21.1989 6.75 21 6.75H20.25Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M5.05526 7.06061C3.80528 6.34633 2.25 7.24889 2.25 8.68856V16.8114C2.25 18.2511 3.80528 19.1536 5.05526 18.4394L12.1628 14.3779C13.4224 13.6581 13.4224 11.8418 12.1628 11.122L5.05526 7.06061Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
