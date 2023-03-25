#[cfg(feature = "FaSolidTurkishLiraSign")]
use leptos::*;
#[cfg(feature = "FaSolidTurkishLiraSign")]
///This icon requires the feature `FaSolidTurkishLiraSign` to be enabled.
#[component]
pub fn TurkishLiraSign(
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
        stroke_witdh = "0" style = style viewBox = "0 0 384 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M96 32c17.7 0 32 14.3 32 32V99.3L247.2 65.2c17-4.9 34.7 5 39.6 22s-5 34.7-22 39.6L128 165.9v29.4l119.2-34.1c17-4.9 34.7 5 39.6 22s-5 34.7-22 39.6L128 261.9V416h63.8c68.2 0 124.4-53.5 127.8-121.6l.4-8c.9-17.7 15.9-31.2 33.6-30.4s31.2 15.9 30.4 33.6l-.4 8C378.5 399.8 294.1 480 191.8 480H96c-17.7 0-32-14.3-32-32V280.1l-23.2 6.6c-17 4.9-34.7-5-39.6-22s5-34.7 22-39.6L64 213.6V184.1l-23.2 6.6c-17 4.9-34.7-5-39.6-22s5-34.7 22-39.6L64 117.6V64c0-17.7 14.3-32 32-32z"
        /> < title > { title } < / title > < / svg >
    }
}
