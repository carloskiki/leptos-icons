#[cfg(feature = "SiZettlr")]
use leptos::*;
#[cfg(feature = "SiZettlr")]
///This icon requires the feature `SiZettlr` to be enabled.
#[component]
pub fn Zettlr(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M2.46.188C.335.57.023 2.534.022 4.295L.02 17.325c0 1.545-.224 3.96.723 5.28 1.201 1.673 3.567 1.355 5.365 1.351l8.31.03c1.61-.003 5.626.104 7.2-.23 2.115-.448 2.326-2.31 2.328-4.082l.01-13.634c.001-1.532.311-3.425-.68-4.71C22.021-.296 19.557.025 17.744.026L13.24.043c-1.614 0-9.195-.14-10.782.145m14.249 3.665c0 .528.185 1.466-.037 1.947-.11.239-.49.384-.703.518-.496.315-.961.676-1.418 1.044-1.477 1.185-3.034 2.818-3.428 4.74-.182.895-.164 1.988.59 2.626 1.21 1.023 3.035-.024 4.317.987 1.337 1.053 1.14 3.071.37 4.39-.184.316-.417.797-.75.982-.23.13-.598.064-.853.064h-1.85c.057-.37.35-.58.575-.862.374-.47.673-.984.668-1.6-.01-1.157-1.127-1.17-1.983-1.17-1.518 0-3.296-.216-4.073-1.724-1.337-2.595.33-5.731 2.105-7.633.695-.746 1.684-1.333 2.276-2.154h-4.07V3.853z"
        /> < title > { title } < / title > < / svg >
    }
}
