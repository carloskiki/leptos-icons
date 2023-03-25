#[cfg(feature = "FaRegularKeyboard")]
use leptos::*;
#[cfg(feature = "FaRegularKeyboard")]
///This icon requires the feature `FaRegularKeyboard` to be enabled.
#[component]
pub fn Keyboard(
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
        "M64 112c-8.8 0-16 7.2-16 16V384c0 8.8 7.2 16 16 16H512c8.8 0 16-7.2 16-16V128c0-8.8-7.2-16-16-16H64zM0 128C0 92.7 28.7 64 64 64H512c35.3 0 64 28.7 64 64V384c0 35.3-28.7 64-64 64H64c-35.3 0-64-28.7-64-64V128zM176 320H400c8.8 0 16 7.2 16 16v16c0 8.8-7.2 16-16 16H176c-8.8 0-16-7.2-16-16V336c0-8.8 7.2-16 16-16zm-72-72c0-8.8 7.2-16 16-16h16c8.8 0 16 7.2 16 16v16c0 8.8-7.2 16-16 16H120c-8.8 0-16-7.2-16-16V248zm16-96h16c8.8 0 16 7.2 16 16v16c0 8.8-7.2 16-16 16H120c-8.8 0-16-7.2-16-16V168c0-8.8 7.2-16 16-16zm64 96c0-8.8 7.2-16 16-16h16c8.8 0 16 7.2 16 16v16c0 8.8-7.2 16-16 16H200c-8.8 0-16-7.2-16-16V248zm16-96h16c8.8 0 16 7.2 16 16v16c0 8.8-7.2 16-16 16H200c-8.8 0-16-7.2-16-16V168c0-8.8 7.2-16 16-16zm64 96c0-8.8 7.2-16 16-16h16c8.8 0 16 7.2 16 16v16c0 8.8-7.2 16-16 16H280c-8.8 0-16-7.2-16-16V248zm16-96h16c8.8 0 16 7.2 16 16v16c0 8.8-7.2 16-16 16H280c-8.8 0-16-7.2-16-16V168c0-8.8 7.2-16 16-16zm64 96c0-8.8 7.2-16 16-16h16c8.8 0 16 7.2 16 16v16c0 8.8-7.2 16-16 16H360c-8.8 0-16-7.2-16-16V248zm16-96h16c8.8 0 16 7.2 16 16v16c0 8.8-7.2 16-16 16H360c-8.8 0-16-7.2-16-16V168c0-8.8 7.2-16 16-16zm64 96c0-8.8 7.2-16 16-16h16c8.8 0 16 7.2 16 16v16c0 8.8-7.2 16-16 16H440c-8.8 0-16-7.2-16-16V248zm16-96h16c8.8 0 16 7.2 16 16v16c0 8.8-7.2 16-16 16H440c-8.8 0-16-7.2-16-16V168c0-8.8 7.2-16 16-16z"
        /> < title > { title } < / title > < / svg >
    }
}
