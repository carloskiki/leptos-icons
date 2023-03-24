#[cfg(feature = "TiBatteryFull")]
use leptos::*;
#[cfg(feature = "TiBatteryFull")]
///This icon requires the feature `TiBatteryFull` to be enabled.
#[component]
pub fn BatteryFull(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = { size.clone() } height = { size
        } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M9 16c-.552 0-1-.447-1-1v-4c0-.553.448-1 1-1s1 .447 1 1v4c0 .553-.448 1-1 1zM6 16c-.552 0-1-.447-1-1v-4c0-.553.448-1 1-1s1 .447 1 1v4c0 .553-.448 1-1 1zM15 16c-.552 0-1-.447-1-1v-4c0-.553.448-1 1-1s1 .447 1 1v4c0 .553-.448 1-1 1zM12 16c-.552 0-1-.447-1-1v-4c0-.553.448-1 1-1s1 .447 1 1v4c0 .553-.448 1-1 1zM19 10c0-1.654-1.346-3-3-3h-11c-1.654 0-3 1.346-3 3v6c0 1.654 1.346 3 3 3h11c1.654 0 3-1.346 3-3 1.104 0 2-.896 2-2v-2c0-1.104-.896-2-2-2zm-2 6c0 .552-.449 1-1 1h-11c-.551 0-1-.448-1-1v-6c0-.552.449-1 1-1h11c.551 0 1 .448 1 1v6z"
        /> < title > { title } < / title > < / svg >
    }
}
