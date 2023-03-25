#[cfg(feature = "SiSnapcraft")]
use leptos::*;
#[cfg(feature = "SiSnapcraft")]
///This icon requires the feature `SiSnapcraft` to be enabled.
#[component]
pub fn Snapcraft(
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
        "M13.804 13.367V5.69l5.292 2.362-5.292 5.315zM3.701 23.514l6.49-12.22 2.847 2.843L3.7 23.514zM0 .486l13.355 4.848v8.484L0 .486zM21.803 5.334H14.11L24 9.748z"
        /> < title > { title } < / title > < / svg >
    }
}
