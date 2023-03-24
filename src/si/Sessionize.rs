#[cfg(feature = "SiSessionize")]
use leptos::*;
#[cfg(feature = "SiSessionize")]
///This icon requires the feature `SiSessionize` to be enabled.
#[component]
pub fn Sessionize(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M12 0c6.628 0 12 5.372 12 12v10c0 1.097-.903 2-2 2h-7.5l-.001-.169c-.049-2.894-1.347-4.902-3.709-5.96L24 12l-.32-.109c-2.858-.999-5.251-2.462-7.18-4.391-1.928-1.928-3.392-4.322-4.391-7.181L12 0 4 18c0 .667.167 1.167.5 1.5.334.334.834.5 1.5.5l.187.001c3.771.04 5.313 1.295 5.313 3.999H2c-1.097 0-2-.903-2-2V2C0 .903.903 0 2 0h10Zm7.207 4.793c-.781-.781-1.73-1.097-2.121-.707-.39.39-.074 1.34.707 2.121.781.781 1.731 1.098 2.121.707.391-.39.074-1.34-.707-2.121Z"
        /> < title > { title } < / title > < / svg >
    }
}
