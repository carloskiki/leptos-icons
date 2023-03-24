#[cfg(feature = "IoTodaySharp")]
use leptos::*;
#[cfg(feature = "IoTodaySharp")]
///This icon requires the feature `IoTodaySharp` to be enabled.
#[component]
pub fn TodaySharp(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M32,456a24,24,0,0,0,24,24H456a24,24,0,0,0,24-24V176H32Zm80-238.86a9.14,9.14,0,0,1,9.14-9.14H230.86a9.14,9.14,0,0,1,9.14,9.14V326.86a9.14,9.14,0,0,1-9.14,9.14H121.14a9.14,9.14,0,0,1-9.14-9.14Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M456,64H400.08V32h-48V64H159.92V32h-48V64H56A23.8,23.8,0,0,0,32,87.77V144H480V87.77A23.8,23.8,0,0,0,456,64Z"
        /> < title > { title } < / title > < / svg >
    }
}
