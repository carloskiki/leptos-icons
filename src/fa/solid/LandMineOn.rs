#[cfg(feature = "FaSolidLandMineOn")]
use leptos::*;
#[cfg(feature = "FaSolidLandMineOn")]
///This icon requires the feature `FaSolidLandMineOn` to be enabled.
#[component]
pub fn LandMineOn(
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
        stroke_witdh = "0" style = style viewBox = "0 0 576 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M312 24V168c0 13.3-10.7 24-24 24s-24-10.7-24-24V24c0-13.3 10.7-24 24-24s24 10.7 24 24zM160 320c0-17.7 14.3-32 32-32H384c17.7 0 32 14.3 32 32v32H160V320zM82.7 410.5c8.1-16.3 24.8-26.5 42.9-26.5H450.3c18.2 0 34.8 10.3 42.9 26.5l27.6 55.2C531.5 487 516 512 492.2 512H83.8c-23.8 0-39.3-25-28.6-46.3l27.6-55.2zM4.3 138.3c7.5-10.9 22.5-13.6 33.4-6.1l104 72c10.9 7.5 13.6 22.5 6.1 33.4s-22.5 13.6-33.4 6.1l-104-72c-10.9-7.5-13.6-22.5-6.1-33.4zm534.1-6.1c10.9-7.5 25.8-4.8 33.4 6.1s4.8 25.8-6.1 33.4l-104 72c-10.9 7.5-25.8 4.8-33.4-6.1s-4.8-25.8 6.1-33.4l104-72z"
        /> < title > { title } < / title > < / svg >
    }
}
