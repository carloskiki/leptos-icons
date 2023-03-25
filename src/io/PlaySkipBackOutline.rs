#[cfg(feature = "IoPlaySkipBackOutline")]
use leptos::*;
#[cfg(feature = "IoPlaySkipBackOutline")]
///This icon requires the feature `IoPlaySkipBackOutline` to be enabled.
#[component]
pub fn PlaySkipBackOutline(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M400,111V401c0,17.44-17,28.52-31,20.16L121.09,272.79c-12.12-7.25-12.12-26.33,0-33.58L369,90.84C383,82.48,400,93.56,400,111Z"
        style = "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" />< line
        xmlns = "http://www.w3.org/2000/svg" x1 = "112" y1 = "80" x2 = "112" y2 = "432"
        style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px"
        /> < title > { title } < / title > < / svg >
    }
}
