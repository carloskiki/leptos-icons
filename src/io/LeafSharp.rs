#[cfg(feature = "IoLeafSharp")]
use leptos::*;
#[cfg(feature = "IoLeafSharp")]
///This icon requires the feature `IoLeafSharp` to be enabled.
#[component]
pub fn LeafSharp(
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
        "M150.38,253.68l21.94-23.3,11.65,11c73.63,69.36,147.51,111.56,234.45,133.07,11.73-32,12.77-67.22,2.64-101.58-13.44-45.59-44.74-85.31-90.49-114.86-40.25-26-76.6-32.09-115.09-38.54-21.12-3.54-43-7.2-66.85-14.43C104.85,91.76,58.94,52.3,58.48,51.91L33.4,30.15,32,63.33c-.1,2.56-2.42,63.57,14.22,147.77,17.58,89,50.24,155.85,97.07,198.63,38,34.69,87.62,53.9,136.93,53.9A185.88,185.88,0,0,0,308,461.56c41.72-6.32,76.43-27.27,96-57.75-89.5-23.28-165.95-67.55-242-139.16Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M467.43,384.19c-16.83-2.59-33.13-5.84-49-9.77A158.49,158.49,0,0,1,406.3,400.1c-.74,1.25-1.51,2.49-2.29,3.71a583.43,583.43,0,0,0,58.55,12l15.82,2.44,4.86-31.63Z"
        /> < title > { title } < / title > < / svg >
    }
}
