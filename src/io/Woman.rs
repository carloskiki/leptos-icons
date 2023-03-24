#[cfg(feature = "IoWoman")]
use leptos::*;
#[cfg(feature = "IoWoman")]
///This icon requires the feature `IoWoman` to be enabled.
#[component]
pub fn Woman(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = { size.clone() } height = { size } > < circle xmlns =
        "http://www.w3.org/2000/svg" cx = "255.75" cy = "56" r = "56" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M394.63,277.9,384.3,243.49s0-.07,0-.11l-22.46-74.86h-.05l-2.51-8.45a44.87,44.87,0,0,0-43-32.08h-120a44.84,44.84,0,0,0-43,32.08l-2.51,8.45h-.06l-22.46,74.86s0,.07,0,.11L117.88,277.9c-3.12,10.39,2.3,21.66,12.57,25.14a20,20,0,0,0,25.6-13.18l25.58-85.25h0l2.17-7.23A8,8,0,0,1,199.33,200a7.78,7.78,0,0,1-.17,1.61v0L155.43,347.4A16,16,0,0,0,170.75,368h29V482.69c0,16.46,10.53,29.31,24,29.31s24-12.85,24-29.31V368h16V482.69c0,16.46,10.53,29.31,24,29.31s24-12.85,24-29.31V368h30a16,16,0,0,0,15.33-20.6L313.34,201.59a7.52,7.52,0,0,1-.16-1.59,8,8,0,0,1,15.54-2.63l2.17,7.23h0l25.57,85.25A20,20,0,0,0,382.05,303C392.32,299.56,397.74,288.29,394.63,277.9Z"
        /> < title > { title } < / title > < / svg >
    }
}
