#[cfg(feature = "FaSolidAustralSign")]
use leptos::*;
#[cfg(feature = "FaSolidAustralSign")]
///This icon requires the feature `FaSolidAustralSign` to be enabled.
#[component]
pub fn AustralSign(
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
        stroke_witdh = "0" style = style viewBox = "0 0 448 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M253.5 51.7C248.6 39.8 236.9 32 224 32s-24.6 7.8-29.5 19.7L122.7 224H32c-17.7 0-32 14.3-32 32s14.3 32 32 32H96L82.7 320H32c-17.7 0-32 14.3-32 32s14.3 32 32 32H56L34.5 435.7c-6.8 16.3 .9 35 17.2 41.8s35-.9 41.8-17.2L125.3 384H322.7l31.8 76.3c6.8 16.3 25.5 24 41.8 17.2s24-25.5 17.2-41.8L392 384h24c17.7 0 32-14.3 32-32s-14.3-32-32-32H365.3L352 288h64c17.7 0 32-14.3 32-32s-14.3-32-32-32H325.3L253.5 51.7zM256 224H192l32-76.8L256 224zm-90.7 64H282.7L296 320H152l13.3-32z"
        /> < title > { title } < / title > < / svg >
    }
}
