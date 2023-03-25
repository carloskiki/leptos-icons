#[cfg(feature = "FaBrandsCodiepie")]
use leptos::*;
#[cfg(feature = "FaBrandsCodiepie")]
///This icon requires the feature `FaBrandsCodiepie` to be enabled.
#[component]
pub fn Codiepie(
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
        stroke_witdh = "0" style = style viewBox = "0 0 472 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M422.5 202.9c30.7 0 33.5 53.1-.3 53.1h-10.8v44.3h-26.6v-97.4h37.7zM472 352.6C429.9 444.5 350.4 504 248 504 111 504 0 393 0 256S111 8 248 8c97.4 0 172.8 53.7 218.2 138.4l-186 108.8L472 352.6zm-38.5 12.5l-60.3-30.7c-27.1 44.3-70.4 71.4-122.4 71.4-82.5 0-149.2-66.7-149.2-148.9 0-82.5 66.7-149.2 149.2-149.2 48.4 0 88.9 23.5 116.9 63.4l59.5-34.6c-40.7-62.6-104.7-100-179.2-100-121.2 0-219.5 98.3-219.5 219.5S126.8 475.5 248 475.5c78.6 0 146.5-42.1 185.5-110.4z"
        /> < title > { title } < / title > < / svg >
    }
}
