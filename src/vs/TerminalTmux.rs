#[cfg(feature = "VsTerminalTmux")]
use leptos::*;
#[cfg(feature = "VsTerminalTmux")]
///This icon requires the feature `VsTerminalTmux` to be enabled.
#[component]
pub fn TerminalTmux(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M13.5 1h-12l-.5.5v13l.5.5h12l.5-.5v-13l-.5-.5zM7 7.5V13H2V2h5v5.5zm6 5.5H8V8h5v5zm0-6H8V2h5v5z"
        /> < title > { title } < / title > < / svg >
    }
}
