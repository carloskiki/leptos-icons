#[cfg(feature = "FaSolidHelmetUn")]
use leptos::*;
#[cfg(feature = "FaSolidHelmetUn")]
///This icon requires the feature `FaSolidHelmetUn` to be enabled.
#[component]
pub fn HelmetUn(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M479.5 224C471.2 98.9 367.2 0 240 0C107.5 0 0 107.5 0 240v56.3C0 344.8 39.2 384 87.7 384H200h14.9L343.5 505.4c4.5 4.2 10.4 6.6 16.5 6.6h96c13.3 0 24-10.7 24-24s-10.7-24-24-24H369.5l-1.5-1.5V288h80 32c17.7 0 32-14.3 32-32s-14.3-32-32-32h-.5zM320 417.2l-78-73.7L274.4 288H320V417.2zM285.3 103.1l34.7 52V112c0-8.8 7.2-16 16-16s16 7.2 16 16v96c0 7.1-4.6 13.3-11.4 15.3s-14-.6-17.9-6.4l-34.7-52V208c0 8.8-7.2 16-16 16s-16-7.2-16-16V112c0-7.1 4.6-13.3 11.4-15.3s14 .6 17.9 6.4zM160 112v64c0 8.8 7.2 16 16 16s16-7.2 16-16V112c0-8.8 7.2-16 16-16s16 7.2 16 16v64c0 26.5-21.5 48-48 48s-48-21.5-48-48V112c0-8.8 7.2-16 16-16s16 7.2 16 16z"
        /> < title > { title } < / title > < / svg >
    }
}
