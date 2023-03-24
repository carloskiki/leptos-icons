#[cfg(feature = "IoTelescopeOutline")]
use leptos::*;
#[cfg(feature = "IoTelescopeOutline")]
///This icon requires the feature `IoTelescopeOutline` to be enabled.
#[component]
pub fn TelescopeOutline(
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
    let style = format!("{} color: {};", style, color);
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" fill = "none" stroke = "#000" stroke - linecap =
        "round" stroke - linejoin = "round" stroke - width = "32" d =
        "M39.93,327.56l-4.71-8.13A24,24,0,0,1,44,286.64l86.87-50.07a16,16,0,0,1,21.89,5.86l12.71,22a16,16,0,0,1-5.86,21.85L72.76,336.35A24.06,24.06,0,0,1,39.93,327.56Z"
        />< path xmlns = "http://www.w3.org/2000/svg" fill = "none" stroke = "#000"
        stroke - linecap = "round" stroke - linejoin = "round" stroke - width = "32" d =
        "M170.68,273.72,147.12,233a24,24,0,0,1,8.8-32.78l124.46-71.75a16,16,0,0,1,21.89,5.86l31.57,54.59A16,16,0,0,1,328,210.76L203.51,282.5A24,24,0,0,1,170.68,273.72Z"
        />< path xmlns = "http://www.w3.org/2000/svg" fill = "none" stroke = "#000"
        stroke - linecap = "round" stroke - linejoin = "round" stroke - width = "32" d =
        "M341.85,202.21l-46.51-80.43A24,24,0,0,1,304.14,89l93.29-53.78A24.07,24.07,0,0,1,430.27,44l46.51,80.43a24,24,0,0,1-8.8,32.79L374.69,211A24.06,24.06,0,0,1,341.85,202.21Z"
        />< line xmlns = "http://www.w3.org/2000/svg" fill = "none" stroke = "#000"
        stroke - linecap = "round" stroke - linejoin = "round" stroke - width = "32" x1 =
        "127.59" y1 = "480" x2 = "223.73" y2 = "272.01" />< line xmlns =
        "http://www.w3.org/2000/svg" fill = "none" stroke = "#000" stroke - linecap =
        "round" stroke - linejoin = "round" stroke - width = "32" x1 = "271.8" y1 =
        "256.02" x2 = "368.55" y2 = "448" /> < title > { title } < / title > < / svg >
    }
}
