#[cfg(feature = "BiSolidHappyHeartEyes")]
use leptos::*;
#[cfg(feature = "BiSolidHappyHeartEyes")]
///This icon requires the feature `BiSolidHappyHeartEyes` to be enabled.
#[component]
pub fn HappyHeartEyes(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
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
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M12 2C6.486 2 2 6.486 2 12s4.486 10 10 10 10-4.486 10-10S17.514 2 12 2zM6.435 8.467A1.49 1.49 0 0 1 8.502 8.4a1.49 1.49 0 0 1 2.065.033c.597.592.604 1.521.018 2.118l-2.05 2.083-2.082-2.05a1.484 1.484 0 0 1-.018-2.117zM12 18c-4 0-5-4-5-4h10s-1 4-5 4zm5.585-7.449-2.05 2.083-2.082-2.05a1.485 1.485 0 0 1-.019-2.117 1.49 1.49 0 0 1 2.068-.067 1.49 1.49 0 0 1 2.065.033c.597.591.605 1.521.018 2.118z"
        /> < title > { title } < / title > < / svg >
    }
}
