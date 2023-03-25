#[cfg(feature = "CgModem")]
use leptos::*;
#[cfg(feature = "CgModem")]
///This icon requires the feature `CgModem` to be enabled.
#[component]
pub fn Modem(
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
        "M18 16.6341C18 17.1864 17.5523 17.6341 17 17.6341C16.4477 17.6341 16 17.1864 16 16.6341C16 16.0819 16.4477 15.6341 17 15.6341C17.5523 15.6341 18 16.0819 18 16.6341Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M5.86603 3.13414C5.38773 2.858 4.77614 3.02187 4.5 3.50017C4.22386 3.97846 4.38773 4.59005 4.86603 4.86619L18.3205 12.6341H2V16.6341C2 18.8433 3.79086 20.6341 6 20.6341H18C20.2091 20.6341 22 18.8433 22 16.6341V12.6341L5.86603 3.13414ZM20 14.6341H4V16.6341C4 17.7387 4.89543 18.6341 6 18.6341H18C19.1046 18.6341 20 17.7387 20 16.6341V14.6341Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
