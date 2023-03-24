#[cfg(feature = "VsGraphScatter")]
use leptos::*;
#[cfg(feature = "VsGraphScatter")]
///This icon requires the feature `VsGraphScatter` to be enabled.
#[component]
pub fn GraphScatter(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M15 13V14H1.5L1 13.5V0H2V13H15Z" />< rect xmlns = "http://www.w3.org/2000/svg" x
        = "5" y = "2" width = "2" height = "2" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "12" y = "1" width = "2" height = "2" />< rect
        xmlns = "http://www.w3.org/2000/svg" x = "8" y = "5" width = "2" height = "2" /><
        rect xmlns = "http://www.w3.org/2000/svg" x = "5" y = "9" width = "2" height =
        "2" />< rect xmlns = "http://www.w3.org/2000/svg" x = "12" y = "8" width = "2"
        height = "2" /> < title > { title } < / title > < / svg >
    }
}
