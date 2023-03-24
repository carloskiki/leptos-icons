#[cfg(feature = "FaSolidLandmarkDome")]
use leptos::*;
#[cfg(feature = "FaSolidLandmarkDome")]
///This icon requires the feature `FaSolidLandmarkDome` to be enabled.
#[component]
pub fn LandmarkDome(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M248 0h16c13.3 0 24 10.7 24 24V34.7C368.4 48.1 431.9 111.6 445.3 192H448c17.7 0 32 14.3 32 32s-14.3 32-32 32H64c-17.7 0-32-14.3-32-32s14.3-32 32-32h2.7C80.1 111.6 143.6 48.1 224 34.7V24c0-13.3 10.7-24 24-24zM64 288h64V416h40V288h64V416h48V288h64V416h40V288h64V420.3c.6 .3 1.2 .7 1.7 1.1l48 32c11.7 7.8 17 22.4 12.9 35.9S494.1 512 480 512H32c-14.1 0-26.5-9.2-30.6-22.7s1.1-28.1 12.9-35.9l48-32c.6-.4 1.2-.7 1.8-1.1V288z"
        /> < title > { title } < / title > < / svg >
    }
}
