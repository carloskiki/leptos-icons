#[cfg(feature = "HiLgOutlineQrCode")]
use leptos::*;
#[cfg(feature = "HiLgOutlineQrCode")]
///This icon requires the feature `HiLgOutlineQrCode` to be enabled.
#[component]
pub fn QrCode(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M3.75 4.875C3.75 4.25368 4.25368 3.75 4.875 3.75H9.375C9.99632 3.75 10.5 4.25368 10.5 4.875V9.375C10.5 9.99632 9.99632 10.5 9.375 10.5H4.875C4.25368 10.5 3.75 9.99632 3.75 9.375V4.875Z"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3.75 14.625C3.75 14.0037 4.25368 13.5 4.875 13.5H9.375C9.99632 13.5 10.5 14.0037 10.5 14.625V19.125C10.5 19.7463 9.99632 20.25 9.375 20.25H4.875C4.25368 20.25 3.75 19.7463 3.75 19.125V14.625Z"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M13.5 4.875C13.5 4.25368 14.0037 3.75 14.625 3.75H19.125C19.7463 3.75 20.25 4.25368 20.25 4.875V9.375C20.25 9.99632 19.7463 10.5 19.125 10.5H14.625C14.0037 10.5 13.5 9.99632 13.5 9.375V4.875Z"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M6.75 6.75H7.5V7.5H6.75V6.75Z" stroke = "#0F172A" stroke - width = "1.5" stroke
        - linecap = "round" stroke - linejoin = "round" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M6.75 16.5H7.5V17.25H6.75V16.5Z" stroke =
        "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke - linejoin =
        "round" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M16.5 6.75H17.25V7.5H16.5V6.75Z" stroke = "#0F172A" stroke - width = "1.5"
        stroke - linecap = "round" stroke - linejoin = "round" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M13.5 13.5H14.25V14.25H13.5V13.5Z" stroke =
        "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke - linejoin =
        "round" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M13.5 19.5H14.25V20.25H13.5V19.5Z" stroke = "#0F172A" stroke - width = "1.5"
        stroke - linecap = "round" stroke - linejoin = "round" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19.5 13.5H20.25V14.25H19.5V13.5Z" stroke =
        "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke - linejoin =
        "round" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M19.5 19.5H20.25V20.25H19.5V19.5Z" stroke = "#0F172A" stroke - width = "1.5"
        stroke - linecap = "round" stroke - linejoin = "round" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16.5 16.5H17.25V17.25H16.5V16.5Z" stroke =
        "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke - linejoin =
        "round" /> < title > { title } < / title > < / svg >
    }
}
