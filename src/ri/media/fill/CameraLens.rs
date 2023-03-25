#[cfg(feature = "RiMediaFillCameraLens")]
use leptos::*;
#[cfg(feature = "RiMediaFillCameraLens")]
///This icon requires the feature `RiMediaFillCameraLens` to be enabled.
#[component]
pub fn CameraLens(
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
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M9.827 21.763L14.31 14l3.532 6.117A9.955 9.955 0 0 1 12 22c-.746 0-1.473-.082-2.173-.237zM7.89 21.12A10.028 10.028 0 0 1 2.458 15h8.965L7.89 21.119zM2.05 13a9.964 9.964 0 0 1 2.583-7.761L9.112 13H2.05zm4.109-9.117A9.955 9.955 0 0 1 12 2c.746 0 1.473.082 2.173.237L9.69 10 6.159 3.883zM16.11 2.88A10.028 10.028 0 0 1 21.542 9h-8.965l3.533-6.119zM21.95 11a9.964 9.964 0 0 1-2.583 7.761L14.888 11h7.064z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
