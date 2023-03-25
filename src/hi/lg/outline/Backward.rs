#[cfg(feature = "HiLgOutlineBackward")]
use leptos::*;
#[cfg(feature = "HiLgOutlineBackward")]
///This icon requires the feature `HiLgOutlineBackward` to be enabled.
#[component]
pub fn Backward(
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
        "M21 16.811C21 17.6748 20.0668 18.2164 19.3168 17.7878L12.2094 13.7264C11.4536 13.2945 11.4536 12.2047 12.2094 11.7728L19.3168 7.71141C20.0668 7.28285 21 7.82439 21 8.68819V16.811Z"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M11.25 16.811C11.25 17.6748 10.3168 18.2164 9.56684 17.7878L2.45935 13.7264C1.70356 13.2945 1.70356 12.2047 2.45935 11.7728L9.56684 7.71141C10.3168 7.28285 11.25 7.82439 11.25 8.68819L11.25 16.811Z"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" /> < title > { title } < / title > < / svg >
    }
}
