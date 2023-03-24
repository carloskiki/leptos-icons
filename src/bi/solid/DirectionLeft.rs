#[cfg(feature = "BiSolidDirectionLeft")]
use leptos::*;
#[cfg(feature = "BiSolidDirectionLeft")]
///This icon requires the feature `BiSolidDirectionLeft` to be enabled.
#[component]
pub fn DirectionLeft(
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
        "m21.707 11.294-8.978-9a1.001 1.001 0 0 0-1.415-.002l-9.021 9a1 1 0 0 0 0 1.416l9.021 9c.39.389 1.026.388 1.415-.002l8.978-9a.998.998 0 0 0 0-1.412zM15 16h-2v-4h-3v2l-3-3 3-3v2h5v6z"
        /> < title > { title } < / title > < / svg >
    }
}
