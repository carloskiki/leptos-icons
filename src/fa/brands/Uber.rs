#[cfg(feature = "FaBrandsUber")]
use leptos::*;
#[cfg(feature = "FaBrandsUber")]
///This icon requires the feature `FaBrandsUber` to be enabled.
#[component]
pub fn Uber(
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
        stroke_witdh = "0" style = style viewBox = "0 0 448 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M414.1 32H33.9C15.2 32 0 47.2 0 65.9V446c0 18.8 15.2 34 33.9 34H414c18.7 0 33.9-15.2 33.9-33.9V65.9C448 47.2 432.8 32 414.1 32zM237.6 391.1C163 398.6 96.4 344.2 88.9 269.6h94.4V290c0 3.7 3 6.8 6.8 6.8H258c3.7 0 6.8-3 6.8-6.8v-67.9c0-3.7-3-6.8-6.8-6.8h-67.9c-3.7 0-6.8 3-6.8 6.8v20.4H88.9c7-69.4 65.4-122.2 135.1-122.2 69.7 0 128.1 52.8 135.1 122.2 7.5 74.5-46.9 141.1-121.5 148.6z"
        /> < title > { title } < / title > < / svg >
    }
}
