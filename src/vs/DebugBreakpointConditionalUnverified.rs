#[cfg(feature = "VsDebugBreakpointConditionalUnverified")]
use leptos::*;
#[cfg(feature = "VsDebugBreakpointConditionalUnverified")]
///This icon requires the feature `VsDebugBreakpointConditionalUnverified` to be enabled.
#[component]
pub fn DebugBreakpointConditionalUnverified(
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
        "M5.778 4.674a4 4 0 1 1 4.444 6.652 4 4 0 0 1-4.444-6.652zm.694 5.612a2.75 2.75 0 1 0 3.056-4.572 2.75 2.75 0 0 0-3.056 4.572zM9.5 6.5h-3v1h3v-1zm0 2h-3v1h3v-1z"
        /> < title > { title } < / title > < / svg >
    }
}
