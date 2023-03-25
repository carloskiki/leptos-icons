#[cfg(feature = "FaSolidUpLong")]
use leptos::*;
#[cfg(feature = "FaSolidUpLong")]
///This icon requires the feature `FaSolidUpLong` to be enabled.
#[component]
pub fn UpLong(
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
        stroke_witdh = "0" style = style viewBox = "0 0 320 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M318 177.5c3.8-8.8 2-19-4.6-26l-136-144C172.9 2.7 166.6 0 160 0s-12.9 2.7-17.4 7.5l-136 144c-6.6 7-8.4 17.2-4.6 26S14.4 192 24 192H96l0 288c0 17.7 14.3 32 32 32h64c17.7 0 32-14.3 32-32l0-288h72c9.6 0 18.2-5.7 22-14.5z"
        /> < title > { title } < / title > < / svg >
    }
}
