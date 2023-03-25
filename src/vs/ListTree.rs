#[cfg(feature = "VsListTree")]
use leptos::*;
#[cfg(feature = "VsListTree")]
///This icon requires the feature `VsListTree` to be enabled.
#[component]
pub fn ListTree(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < rect xmlns = "http://www.w3.org/2000/svg" x =
        "4" y = "9" width = "9" height = "1" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "4" y = "12" width = "7" height = "1" />< rect
        xmlns = "http://www.w3.org/2000/svg" x = "4" y = "6" width = "10" height = "1"
        />< rect xmlns = "http://www.w3.org/2000/svg" x = "1" y = "3" width = "11" height
        = "1" />< rect xmlns = "http://www.w3.org/2000/svg" x = "4" y = "4" width = "1"
        height = "9" /> < title > { title } < / title > < / svg >
    }
}
