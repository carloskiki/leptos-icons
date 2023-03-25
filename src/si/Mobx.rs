#[cfg(feature = "SiMobx")]
use leptos::*;
#[cfg(feature = "SiMobx")]
///This icon requires the feature `SiMobx` to be enabled.
#[component]
pub fn Mobx(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M1.402 0C.625 0 0 .625 0 1.402v21.196C0 23.375.625 24 1.402 24h21.196c.777 0 1.402-.625 1.402-1.402V1.402C24 .625 23.375 0 22.598 0zm2.882 5.465h3.038v13.068H4.284v-.986h1.863V6.45H4.284zm12.394 0h3.038v.985h-1.863v11.098h1.863v.986h-3.038zm-7.856 3.55h1.35c.108.441.234.914.378 1.418.153.495.31.99.472 1.485.171.486.342.958.513 1.417.171.46.333.869.486 1.229.153-.36.315-.77.486-1.229.171-.459.338-.931.5-1.417.17-.495.328-.99.472-1.485.153-.504.284-.977.392-1.418h1.296a34.295 34.295 0 0 1-1.242 3.78 56.44 56.44 0 0 1-1.364 3.24h-1.134a63.191 63.191 0 0 1-1.377-3.24 36.226 36.226 0 0 1-1.228-3.78Z"
        /> < title > { title } < / title > < / svg >
    }
}
