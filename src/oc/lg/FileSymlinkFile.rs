#[cfg(feature = "OcLgFileSymlinkFile")]
use leptos::*;
#[cfg(feature = "OcLgFileSymlinkFile")]
///This icon requires the feature `OcLgFileSymlinkFile` to be enabled.
#[component]
pub fn FileSymlinkFile(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M3 3a2 2 0 0 1 2-2h9.982a2 2 0 0 1 1.414.586l4.018 4.018A2 2 0 0 1 21 7.018V21a2 2 0 0 1-2 2H4.75a.75.75 0 0 1 0-1.5H19a.5.5 0 0 0 .5-.5V8.5h-4a2 2 0 0 1-2-2v-4H5a.5.5 0 0 0-.5.5v6.25a.75.75 0 0 1-1.5 0Zm6.308 11.5-2.104-2.236a.751.751 0 0 1 .369-1.255.749.749 0 0 1 .723.227l3.294 3.5a.75.75 0 0 1 0 1.028l-3.294 3.5a.749.749 0 0 1-1.275-.293.751.751 0 0 1 .183-.735L9.308 16H4.09a2.59 2.59 0 0 0-2.59 2.59v3.16a.75.75 0 0 1-1.5 0v-3.16a4.09 4.09 0 0 1 4.09-4.09ZM15 2.5v4a.5.5 0 0 0 .5.5h4a.5.5 0 0 0-.146-.336l-4.018-4.018A.5.5 0 0 0 15 2.5Z"
        /> < title > { title } < / title > < / svg >
    }
}
