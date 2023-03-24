#[cfg(feature = "VsClippy")]
use leptos::*;
#[cfg(feature = "VsClippy")]
///This icon requires the feature `VsClippy` to be enabled.
#[component]
pub fn Clippy(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M7 13.992H4v-9h8v2h1v-2.5l-.5-.5H11v-1h-1a2 2 0 0 0-4 0H4.94v1H3.5l-.5.5v10l.5.5H7v-1zm0-11.2a1 1 0 0 1 .8-.8 1 1 0 0 1 .58.06.94.94 0 0 1 .45.36 1 1 0 1 1-1.75.94 1 1 0 0 1-.08-.56zm7.08 9.46L13 13.342v-5.35h-1v5.34l-1.08-1.08-.71.71 1.94 1.93h.71l1.93-1.93-.71-.71zm-5.92-4.16h.71l1.93 1.93-.71.71-1.08-1.08v5.34h-1v-5.35l-1.08 1.09-.71-.71 1.94-1.93z"
        /> < title > { title } < / title > < / svg >
    }
}
