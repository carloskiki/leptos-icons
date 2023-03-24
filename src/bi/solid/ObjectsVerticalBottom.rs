#[cfg(feature = "BiSolidObjectsVerticalBottom")]
use leptos::*;
#[cfg(feature = "BiSolidObjectsVerticalBottom")]
///This icon requires the feature `BiSolidObjectsVerticalBottom` to be enabled.
#[component]
pub fn ObjectsVerticalBottom(
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
        xmlns = "http://www.w3.org/2000/svg" d = "M2 20h20v2H2z" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "5" y = "2" width = "6" height = "16" rx = "1"
        />< rect xmlns = "http://www.w3.org/2000/svg" x = "13" y = "6" width = "6" height
        = "12" rx = "1" /> < title > { title } < / title > < / svg >
    }
}
