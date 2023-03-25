#[cfg(feature = "SiCocoapods")]
use leptos::*;
#[cfg(feature = "SiCocoapods")]
///This icon requires the feature `SiCocoapods` to be enabled.
#[component]
pub fn Cocoapods(
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
        "M8.812 17.176c-2.968 0-4.956-2.308-4.956-5.176 0-2.705 1.776-5.176 4.91-5.176 2.407 0 3.856 1.445 4.207 3.357h3.95C16.479 6.427 13.51 3.42 8.718 3.42 3.131 3.42 0 7.523 0 12c0 4.57 3.295 8.58 8.766 8.58 4.58 0 7.549-2.822 8.18-6.272h-4.02c-.467 1.609-1.916 2.868-4.114 2.868zM24 12.068l-3.466 8.055-2.38-1.022 2.992-7.055-3.01-7.096 2.433-1.042Z"
        /> < title > { title } < / title > < / svg >
    }
}
