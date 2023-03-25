#[cfg(feature = "VsRunAbove")]
use leptos::*;
#[cfg(feature = "VsRunAbove")]
///This icon requires the feature `VsRunAbove` to be enabled.
#[component]
pub fn RunAbove(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M1.77 1.01L1 1.42v12l.78.42 9-6v-.83l-9.01-6zM2 12.49V2.36l7.6 5.07L2 12.49zM12.15 8h.71l2.5 2.5-.71.71L13 9.56V15h-1V9.55l-1.65 1.65-.7-.7 2.5-2.5z"
        /> < title > { title } < / title > < / svg >
    }
}
