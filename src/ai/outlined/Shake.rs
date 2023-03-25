#[cfg(feature = "AiOutlinedShake")]
use leptos::*;
#[cfg(feature = "AiOutlinedShake")]
///This icon requires the feature `AiOutlinedShake` to be enabled.
#[component]
pub fn Shake(
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
        stroke_witdh = "0" style = style class = "icon" viewBox = "0 0 1024 1024" width =
        size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M324 666a48 48 0 1 0 96 0 48 48 0 1 0-96 0zm616.7-309.6L667.6 83.2C655.2 70.9 638.7 64 621.1 64s-34.1 6.8-46.5 19.2L83.3 574.5a65.85 65.85 0 0 0 0 93.1l273.2 273.2c12.3 12.3 28.9 19.2 46.5 19.2s34.1-6.8 46.5-19.2l491.3-491.3c25.6-25.7 25.6-67.5-.1-93.1zM403 880.1L143.9 621l477.2-477.2 259 259.2L403 880.1zM152.8 373.7a7.9 7.9 0 0 0 11.2 0L373.7 164a7.9 7.9 0 0 0 0-11.2l-38.4-38.4a7.9 7.9 0 0 0-11.2 0L114.3 323.9a7.9 7.9 0 0 0 0 11.2l38.5 38.6zm718.6 276.6a7.9 7.9 0 0 0-11.2 0L650.3 860.1a7.9 7.9 0 0 0 0 11.2l38.4 38.4a7.9 7.9 0 0 0 11.2 0L909.7 700a7.9 7.9 0 0 0 0-11.2l-38.3-38.5z"
        /> < title > { title } < / title > < / svg >
    }
}
