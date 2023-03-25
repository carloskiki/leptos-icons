#[cfg(feature = "HiLgOutlineForward")]
use leptos::*;
#[cfg(feature = "HiLgOutlineForward")]
///This icon requires the feature `HiLgOutlineForward` to be enabled.
#[component]
pub fn Forward(
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
        "M3 8.68819C3 7.82439 3.93317 7.28285 4.68316 7.71141L11.7906 11.7728C12.5464 12.2047 12.5464 13.2945 11.7906 13.7264L4.68316 17.7878C3.93317 18.2164 3 17.6748 3 16.811V8.68819Z"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12.75 8.68819C12.75 7.82439 13.6832 7.28285 14.4332 7.71141L21.5406 11.7728C22.2964 12.2047 22.2964 13.2945 21.5406 13.7264L14.4332 17.7878C13.6832 18.2164 12.75 17.6748 12.75 16.811V8.68819Z"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" /> < title > { title } < / title > < / svg >
    }
}
