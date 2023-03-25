#[cfg(feature = "IoCalendarSharp")]
use leptos::*;
#[cfg(feature = "IoCalendarSharp")]
///This icon requires the feature `IoCalendarSharp` to be enabled.
#[component]
pub fn CalendarSharp(
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
        "M32,456a24,24,0,0,0,24,24H456a24,24,0,0,0,24-24V176H32ZM352,212a4,4,0,0,1,4-4h40a4,4,0,0,1,4,4v40a4,4,0,0,1-4,4H356a4,4,0,0,1-4-4Zm0,80a4,4,0,0,1,4-4h40a4,4,0,0,1,4,4v40a4,4,0,0,1-4,4H356a4,4,0,0,1-4-4Zm-80-80a4,4,0,0,1,4-4h40a4,4,0,0,1,4,4v40a4,4,0,0,1-4,4H276a4,4,0,0,1-4-4Zm0,80a4,4,0,0,1,4-4h40a4,4,0,0,1,4,4v40a4,4,0,0,1-4,4H276a4,4,0,0,1-4-4Zm0,80a4,4,0,0,1,4-4h40a4,4,0,0,1,4,4v40a4,4,0,0,1-4,4H276a4,4,0,0,1-4-4Zm-80-80a4,4,0,0,1,4-4h40a4,4,0,0,1,4,4v40a4,4,0,0,1-4,4H196a4,4,0,0,1-4-4Zm0,80a4,4,0,0,1,4-4h40a4,4,0,0,1,4,4v40a4,4,0,0,1-4,4H196a4,4,0,0,1-4-4Zm-80-80a4,4,0,0,1,4-4h40a4,4,0,0,1,4,4v40a4,4,0,0,1-4,4H116a4,4,0,0,1-4-4Zm0,80a4,4,0,0,1,4-4h40a4,4,0,0,1,4,4v40a4,4,0,0,1-4,4H116a4,4,0,0,1-4-4Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M456,64H400.08V32h-48V64H159.92V32h-48V64H56A23.8,23.8,0,0,0,32,87.77V144H480V87.77A23.8,23.8,0,0,0,456,64Z"
        /> < title > { title } < / title > < / svg >
    }
}
