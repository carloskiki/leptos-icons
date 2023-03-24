#[cfg(feature = "AiTwotoneCompass")]
use leptos::*;
#[cfg(feature = "AiTwotoneCompass")]
///This icon requires the feature `AiTwotoneCompass` to be enabled.
#[component]
pub fn Compass(
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
        stroke_witdh = "0" style = style viewBox = "0 0 1024 1024" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" fill = "#E6E6E6" d =
        "M512 140c-205.4 0-372 166.6-372 372s166.6 372 372 372 372-166.6 372-372-166.6-372-372-372zM327.6 701.7c-2 .9-4.4 0-5.3-2.1-.4-1-.4-2.2 0-3.2L421 470.9 553.1 603l-225.5 98.7zm375.1-375.1L604 552.1 471.9 420l225.5-98.7c2-.9 4.4 0 5.3 2.1.4 1 .4 2.1 0 3.2z"
        />< path xmlns = "http://www.w3.org/2000/svg" fill = "#333" d =
        "M322.3 696.4c-.4 1-.4 2.2 0 3.2.9 2.1 3.3 3 5.3 2.1L553.1 603 421 470.9l-98.7 225.5zm375.1-375.1L471.9 420 604 552.1l98.7-225.5c.4-1.1.4-2.2 0-3.2-.9-2.1-3.3-3-5.3-2.1z"
        />< path xmlns = "http://www.w3.org/2000/svg" fill = "#333" d =
        "M512 64C264.6 64 64 264.6 64 512s200.6 448 448 448 448-200.6 448-448S759.4 64 512 64zm0 820c-205.4 0-372-166.6-372-372s166.6-372 372-372 372 166.6 372 372-166.6 372-372 372z"
        /> < title > { title } < / title > < / svg >
    }
}
