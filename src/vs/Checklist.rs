#[cfg(feature = "VsChecklist")]
use leptos::*;
#[cfg(feature = "VsChecklist")]
///This icon requires the feature `VsChecklist` to be enabled.
#[component]
pub fn Checklist(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M3.75 4.48h-.71L2 3.43l.71-.7.69.68L4.81 2l.71.71-1.77 1.77zM6.99 3h8v1h-8V3zm0 3h8v1h-8V6zm8 3h-8v1h8V9zm-8 3h8v1h-8v-1zM3.04 7.48h.71l1.77-1.77-.71-.7L3.4 6.42l-.69-.69-.71.71 1.04 1.04zm.71 3.01h-.71L2 9.45l.71-.71.69.69 1.41-1.42.71.71-1.77 1.77zm-.71 3.01h.71l1.77-1.77-.71-.71-1.41 1.42-.69-.69-.71.7 1.04 1.05z"
        /> < title > { title } < / title > < / svg >
    }
}
