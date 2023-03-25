#[cfg(feature = "RiOthersLinePlant")]
use leptos::*;
#[cfg(feature = "RiOthersLinePlant")]
///This icon requires the feature `RiOthersLinePlant` to be enabled.
#[component]
pub fn Plant(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0H24V24H0z" />< path d
        =
        "M6 2c2.69 0 5.024 1.517 6.197 3.741C13.374 4.083 15.31 3 17.5 3H21v2.5c0 3.59-2.91 6.5-6.5 6.5H13v1h5v7c0 1.105-.895 2-2 2H8c-1.105 0-2-.895-2-2v-7h5v-2H9c-3.866 0-7-3.134-7-7V2h4zm10 13H8v5h8v-5zm3-10h-1.5C15.015 5 13 7.015 13 9.5v.5h1.5c2.485 0 4.5-2.015 4.5-4.5V5zM6 4H4c0 2.761 2.239 5 5 5h2c0-2.761-2.239-5-5-5z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
