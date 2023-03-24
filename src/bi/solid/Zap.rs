#[cfg(feature = "BiSolidZap")]
use leptos::*;
#[cfg(feature = "BiSolidZap")]
///This icon requires the feature `BiSolidZap` to be enabled.
#[component]
pub fn Zap(
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
        "M20.98 11.802a.995.995 0 0 0-.738-.771l-6.86-1.716 2.537-5.921a.998.998 0 0 0-.317-1.192.996.996 0 0 0-1.234.024l-11 9a1 1 0 0 0 .39 1.744l6.719 1.681-3.345 5.854A1.001 1.001 0 0 0 8 22a.995.995 0 0 0 .6-.2l12-9a1 1 0 0 0 .38-.998z"
        /> < title > { title } < / title > < / svg >
    }
}
