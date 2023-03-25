#[cfg(feature = "SiShotcut")]
use leptos::*;
#[cfg(feature = "SiShotcut")]
///This icon requires the feature `SiShotcut` to be enabled.
#[component]
pub fn Shotcut(
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
        "M0 0h6.667v24H0v-.889h5.778V.889H0V0zm7.556 0v24H24v-.889H8.444V.889H24V0H7.556zm1.388 22.611H24V1.389H8.944v21.222zM5.278 1.389H0v21.222h5.278V1.389z"
        /> < title > { title } < / title > < / svg >
    }
}
