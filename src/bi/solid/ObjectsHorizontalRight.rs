#[cfg(feature = "BiSolidObjectsHorizontalRight")]
use leptos::*;
#[cfg(feature = "BiSolidObjectsHorizontalRight")]
///This icon requires the feature `BiSolidObjectsHorizontalRight` to be enabled.
#[component]
pub fn ObjectsHorizontalRight(
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
        xmlns = "http://www.w3.org/2000/svg" d = "M20 2h2v20h-2z" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "2" y = "13" width = "16" height = "6" rx = "1"
        />< rect xmlns = "http://www.w3.org/2000/svg" x = "6" y = "5" width = "12" height
        = "6" rx = "1" /> < title > { title } < / title > < / svg >
    }
}
