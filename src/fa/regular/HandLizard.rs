#[cfg(feature = "FaRegularHandLizard")]
use leptos::*;
#[cfg(feature = "FaRegularHandLizard")]
///This icon requires the feature `FaRegularHandLizard` to be enabled.
#[component]
pub fn HandLizard(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M72 112c-13.3 0-24 10.7-24 24s10.7 24 24 24H240c35.3 0 64 28.7 64 64s-28.7 64-64 64H136c-13.3 0-24 10.7-24 24s10.7 24 24 24H288c4.5 0 8.9 1.3 12.7 3.6l64 40c7 4.4 11.3 12.1 11.3 20.4v24c0 13.3-10.7 24-24 24s-24-10.7-24-24V413.3L281.1 384H136c-39.8 0-72-32.2-72-72s32.2-72 72-72H240c8.8 0 16-7.2 16-16s-7.2-16-16-16H72c-39.8 0-72-32.2-72-72S32.2 64 72 64H281.6c46.7 0 90.9 21.5 119.7 58.3l78.4 100.1c20.9 26.7 32.3 59.7 32.3 93.7V424c0 13.3-10.7 24-24 24s-24-10.7-24-24V316.1c0-23.2-7.8-45.8-22.1-64.1L363.5 151.9c-19.7-25.2-49.9-39.9-81.9-39.9H72z"
        /> < title > { title } < / title > < / svg >
    }
}
