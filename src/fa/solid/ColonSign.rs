#[cfg(feature = "FaSolidColonSign")]
use leptos::*;
#[cfg(feature = "FaSolidColonSign")]
///This icon requires the feature `FaSolidColonSign` to be enabled.
#[component]
pub fn ColonSign(
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
        "M223 39.8c4.3-17.1-6.1-34.5-23.3-38.8S165.2 7.1 161 24.2L149.9 68.6C64.1 87.8 0 164.4 0 256c0 58.1 25.8 110.2 66.7 145.4L49 472.2c-4.3 17.1 6.1 34.5 23.3 38.8s34.5-6.1 38.8-23.3l13-52.1c9 3.4 18.4 6.2 28 8.2L145 472.2c-4.3 17.1 6.1 34.5 23.3 38.8s34.5-6.1 38.8-23.3l10.4-41.4c33.4-4.4 64.1-17.4 89.8-36.7c14.1-10.6 17-30.7 6.4-44.8s-30.7-17-44.8-6.4c-10.2 7.7-21.7 13.9-34 18.3L289 160c9.4-.3 18.5-4.7 24.6-12.8c10.6-14.1 7.8-34.2-6.4-44.8c-1.1-.8-2.2-1.6-3.3-2.4L319 39.8c4.3-17.1-6.1-34.5-23.3-38.8S261.2 7.1 257 24.2L245.1 71.5c-9.3-2.7-18.8-4.6-28.6-5.9L223 39.8zM131.2 143.3L85.3 326.8C71.9 306.5 64 282.2 64 256c0-48.7 27.2-91 67.2-112.7zm8.6 229.5l61.1-244.6c9.9 .7 19.5 2.5 28.7 5.3l-62 248.1c-9.7-1.9-19-4.8-27.8-8.8z"
        /> < title > { title } < / title > < / svg >
    }
}
