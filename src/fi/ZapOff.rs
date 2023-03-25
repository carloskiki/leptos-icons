#[cfg(feature = "FiZapOff")]
use leptos::*;
#[cfg(feature = "FiZapOff")]
///This icon requires the feature `FiZapOff` to be enabled.
#[component]
pub fn ZapOff(
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
        fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap =
        "round" stroke - linejoin = "round" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < polyline xmlns = "http://www.w3.org/2000/svg"
        points = "12.41 6.75 13 2 10.57 4.92" />< polyline xmlns =
        "http://www.w3.org/2000/svg" points = "18.57 12.91 21 10 15.66 10" />< polyline
        xmlns = "http://www.w3.org/2000/svg" points = "8 8 3 14 12 14 11 22 16 16" /><
        line xmlns = "http://www.w3.org/2000/svg" x1 = "1" y1 = "1" x2 = "23" y2 = "23"
        /> < title > { title } < / title > < / svg >
    }
}
