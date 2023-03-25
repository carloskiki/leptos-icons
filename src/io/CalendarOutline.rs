#[cfg(feature = "IoCalendarOutline")]
use leptos::*;
#[cfg(feature = "IoCalendarOutline")]
///This icon requires the feature `IoCalendarOutline` to be enabled.
#[component]
pub fn CalendarOutline(
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
        height = size xmlns = "http://www.w3.org/2000/svg" > < rect xmlns =
        "http://www.w3.org/2000/svg" fill = "none" stroke = "#000" stroke - linejoin =
        "round" stroke - width = "32" x = "48" y = "80" width = "416" height = "384" rx =
        "48" />< circle xmlns = "http://www.w3.org/2000/svg" cx = "296" cy = "232" r =
        "24" />< circle xmlns = "http://www.w3.org/2000/svg" cx = "376" cy = "232" r =
        "24" />< circle xmlns = "http://www.w3.org/2000/svg" cx = "296" cy = "312" r =
        "24" />< circle xmlns = "http://www.w3.org/2000/svg" cx = "376" cy = "312" r =
        "24" />< circle xmlns = "http://www.w3.org/2000/svg" cx = "136" cy = "312" r =
        "24" />< circle xmlns = "http://www.w3.org/2000/svg" cx = "216" cy = "312" r =
        "24" />< circle xmlns = "http://www.w3.org/2000/svg" cx = "136" cy = "392" r =
        "24" />< circle xmlns = "http://www.w3.org/2000/svg" cx = "216" cy = "392" r =
        "24" />< circle xmlns = "http://www.w3.org/2000/svg" cx = "296" cy = "392" r =
        "24" />< line xmlns = "http://www.w3.org/2000/svg" fill = "none" stroke = "#000"
        stroke - linejoin = "round" stroke - width = "32" stroke - linecap = "round" x1 =
        "128" y1 = "48" x2 = "128" y2 = "80" />< line xmlns =
        "http://www.w3.org/2000/svg" fill = "none" stroke = "#000" stroke - linejoin =
        "round" stroke - width = "32" stroke - linecap = "round" x1 = "384" y1 = "48" x2
        = "384" y2 = "80" />< line xmlns = "http://www.w3.org/2000/svg" fill = "none"
        stroke = "#000" stroke - linejoin = "round" stroke - width = "32" x1 = "464" y1 =
        "160" x2 = "48" y2 = "160" /> < title > { title } < / title > < / svg >
    }
}
