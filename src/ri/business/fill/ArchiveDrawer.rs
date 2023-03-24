#[cfg(feature = "RiBusinessFillArchiveDrawer")]
use leptos::*;
#[cfg(feature = "RiBusinessFillArchiveDrawer")]
///This icon requires the feature `RiBusinessFillArchiveDrawer` to be enabled.
#[component]
pub fn ArchiveDrawer(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M3 13h18v8.002c0 .551-.445.998-.993.998H3.993A.995.995 0 0 1 3 21.002V13zM3 2.998C3 2.447 3.445 2 3.993 2h16.014c.548 0 .993.446.993.998V11H3V2.998zM9 5v2h6V5H9zm0 11v2h6v-2H9z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
