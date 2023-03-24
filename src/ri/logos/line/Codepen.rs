#[cfg(feature = "RiLogosLineCodepen")]
use leptos::*;
#[cfg(feature = "RiLogosLineCodepen")]
///This icon requires the feature `RiLogosLineCodepen` to be enabled.
#[component]
pub fn Codepen(
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
        "none" d = "M0 0h24v24H0z" />< path fill - rule = "nonzero" d =
        "M16.5 13.202L13 15.535v3.596L19.197 15 16.5 13.202zM14.697 12L12 10.202 9.303 12 12 13.798 14.697 12zM20 10.869L18.303 12 20 13.131V10.87zM19.197 9L13 4.869v3.596l3.5 2.333L19.197 9zM7.5 10.798L11 8.465V4.869L4.803 9 7.5 10.798zM4.803 15L11 19.131v-3.596l-3.5-2.333L4.803 15zM4 13.131L5.697 12 4 10.869v2.262zM2 9a1 1 0 0 1 .445-.832l9-6a1 1 0 0 1 1.11 0l9 6A1 1 0 0 1 22 9v6a1 1 0 0 1-.445.832l-9 6a1 1 0 0 1-1.11 0l-9-6A1 1 0 0 1 2 15V9z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
