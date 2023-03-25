#[cfg(feature = "AiOutlinedMonitor")]
use leptos::*;
#[cfg(feature = "AiOutlinedMonitor")]
///This icon requires the feature `AiOutlinedMonitor` to be enabled.
#[component]
pub fn Monitor(
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
        "M692.8 412.7l.2-.2-34.6-44.3a7.97 7.97 0 0 0-11.2-1.4l-50.4 39.3-70.5-90.1a7.97 7.97 0 0 0-11.2-1.4l-37.9 29.7a7.97 7.97 0 0 0-1.4 11.2l70.5 90.2-.2.1 34.6 44.3c2.7 3.5 7.7 4.1 11.2 1.4l50.4-39.3 64.1 82c2.7 3.5 7.7 4.1 11.2 1.4l37.9-29.6c3.5-2.7 4.1-7.7 1.4-11.2l-64.1-82.1zM608 112c-167.9 0-304 136.1-304 304 0 70.3 23.9 135 63.9 186.5L114.3 856.1a8.03 8.03 0 0 0 0 11.3l42.3 42.3c3.1 3.1 8.2 3.1 11.3 0l253.6-253.6C473 696.1 537.7 720 608 720c167.9 0 304-136.1 304-304S775.9 112 608 112zm161.2 465.2C726.2 620.3 668.9 644 608 644s-118.2-23.7-161.2-66.8C403.7 534.2 380 476.9 380 416s23.7-118.2 66.8-161.2c43-43.1 100.3-66.8 161.2-66.8s118.2 23.7 161.2 66.8c43.1 43 66.8 100.3 66.8 161.2s-23.7 118.2-66.8 161.2z"
        /> < title > { title } < / title > < / svg >
    }
}
