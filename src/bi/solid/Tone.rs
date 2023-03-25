#[cfg(feature = "BiSolidTone")]
use leptos::*;
#[cfg(feature = "BiSolidTone")]
///This icon requires the feature `BiSolidTone` to be enabled.
#[component]
pub fn Tone(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M12 2C6.486 2 2 6.486 2 12s4.486 10 10 10 10-4.486 10-10S17.514 2 12 2zm-1 9H4.069a7.965 7.965 0 0 1 .52-2H11v2zm0 4H4.589a7.965 7.965 0 0 1-.52-2H11v2zm0-10.931V7H5.765A7.996 7.996 0 0 1 11 4.069zM5.765 17H11v2.931A7.996 7.996 0 0 1 5.765 17z"
        /> < title > { title } < / title > < / svg >
    }
}
