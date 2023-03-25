#[cfg(feature = "SiWpexplorer")]
use leptos::*;
#[cfg(feature = "SiWpexplorer")]
///This icon requires the feature `SiWpexplorer` to be enabled.
#[component]
pub fn Wpexplorer(
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
        "M24 12A12 12 0 1 1 12 0a12.008 12.008 0 0 1 12 12Zm-1.5 0A10.5 10.5 0 1 0 12 22.5 10.516 10.516 0 0 0 22.5 12ZM7.542 5.841l4.074 1.739-1.739 4.073L5.8 9.914l1.742-4.073Zm5.158 7.926 2.185 4.406H14.2l-2.343-4.687-2.295 4.687h-.656l2.4-5.01-1.046-.441.282-.656 3.215 1.364-.281.67Zm-.553-5.451 3.216 1.378-1.378 3.2-3.2-1.364 1.364-3.215Zm3.764 2.011 2.56 1.082-1.1 2.546-2.545-1.083 1.082-2.545Z"
        /> < title > { title } < / title > < / svg >
    }
}
