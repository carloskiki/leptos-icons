#[cfg(feature = "IoFemaleSharp")]
use leptos::*;
#[cfg(feature = "IoFemaleSharp")]
///This icon requires the feature `IoFemaleSharp` to be enabled.
#[component]
pub fn FemaleSharp(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M430,190c0-95.94-78.06-174-174-174S82,94.06,82,190c0,88.49,66.4,161.77,152,172.61V394H176v44h58v58h44V438h58V394H278V362.61C363.6,351.77,430,278.49,430,190Zm-304,0c0-71.68,58.32-130,130-130s130,58.32,130,130S327.68,320,256,320,126,261.68,126,190Z"
        /> < title > { title } < / title > < / svg >
    }
}
