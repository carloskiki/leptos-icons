#[cfg(feature = "SiScala")]
use leptos::*;
#[cfg(feature = "SiScala")]
///This icon requires the feature `SiScala` to be enabled.
#[component]
pub fn Scala(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M4.589 24c4.537 0 13.81-1.516 14.821-3v-5.729c-.957 1.408-10.284 2.912-14.821 2.912V24zM4.589 16.365c4.537 0 13.81-1.516 14.821-3V7.636c-.957 1.408-10.284 2.912-14.821 2.912v5.817zM4.589 8.729c4.537 0 13.81-1.516 14.821-3V0C18.453 1.408 9.126 2.912 4.589 2.912v5.817z"
        /> < title > { title } < / title > < / svg >
    }
}
