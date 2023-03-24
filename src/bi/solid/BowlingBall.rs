#[cfg(feature = "BiSolidBowlingBall")]
use leptos::*;
#[cfg(feature = "BiSolidBowlingBall")]
///This icon requires the feature `BiSolidBowlingBall` to be enabled.
#[component]
pub fn BowlingBall(
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
        "M12 2C6.486 2 2 6.486 2 12s4.486 10 10 10 10-4.486 10-10S17.514 2 12 2zM6.5 12a1.5 1.5 0 1 1 .001-3.001A1.5 1.5 0 0 1 6.5 12zM9 6.5a1.5 1.5 0 1 1 3.001.001A1.5 1.5 0 0 1 9 6.5zm2.5 6.5a1.5 1.5 0 1 1 .001-3.001A1.5 1.5 0 0 1 11.5 13z"
        /> < title > { title } < / title > < / svg >
    }
}
