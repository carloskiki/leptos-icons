#[cfg(feature = "CgStopwatch")]
use leptos::*;
#[cfg(feature = "CgStopwatch")]
///This icon requires the feature `CgStopwatch` to be enabled.
#[component]
pub fn Stopwatch(
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
        "M18.6214 2.55029L21.4498 5.37872L20.0356 6.79293L17.2071 3.96451L18.6214 2.55029Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12.8225 8.60055H10.8225V12.6005H12.8225V8.60055Z" fill = "currentColor" /><
        path xmlns = "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule =
        "evenodd" d =
        "M5.18633 18.8137C8.70105 22.3285 14.3995 22.3285 17.9143 18.8137C21.429 15.299 21.429 9.60055 17.9143 6.08583C14.3995 2.57111 8.70105 2.57111 5.18633 6.08583C1.67161 9.60054 1.67161 15.299 5.18633 18.8137ZM6.60054 17.3995C9.33422 20.1332 13.7664 20.1332 16.5 17.3995C19.2337 14.6659 19.2337 10.2337 16.5 7.50004C13.7664 4.76637 9.33422 4.76637 6.60054 7.50004C3.86688 10.2337 3.86688 14.6659 6.60054 17.3995Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
