#[cfg(feature = "FiSun")]
use leptos::*;
#[cfg(feature = "FiSun")]
///This icon requires the feature `FiSun` to be enabled.
#[component]
pub fn Sun(
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
        "http://www.w3.org/2000/svg" > < circle xmlns = "http://www.w3.org/2000/svg" cx =
        "12" cy = "12" r = "5" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "12" y1
        = "1" x2 = "12" y2 = "3" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "12"
        y1 = "21" x2 = "12" y2 = "23" />< line xmlns = "http://www.w3.org/2000/svg" x1 =
        "4.22" y1 = "4.22" x2 = "5.64" y2 = "5.64" />< line xmlns =
        "http://www.w3.org/2000/svg" x1 = "18.36" y1 = "18.36" x2 = "19.78" y2 = "19.78"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "1" y1 = "12" x2 = "3" y2 =
        "12" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "21" y1 = "12" x2 = "23"
        y2 = "12" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "4.22" y1 = "19.78"
        x2 = "5.64" y2 = "18.36" />< line xmlns = "http://www.w3.org/2000/svg" x1 =
        "18.36" y1 = "5.64" x2 = "19.78" y2 = "4.22" /> < title > { title } < / title > <
        / svg >
    }
}
