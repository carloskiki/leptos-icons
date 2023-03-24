#[cfg(feature = "VsFlame")]
use leptos::*;
#[cfg(feature = "VsFlame")]
///This icon requires the feature `VsFlame` to be enabled.
#[component]
pub fn Flame(
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
        "http://www.w3.org/2000/svg" d =
        "M9.13 15l-.53-.77a1.85 1.85 0 0 0-.28-2.54 3.51 3.51 0 0 1-1.19-2c-1.56 2.23-.75 3.46 0 4.55l-.55.76A4.4 4.4 0 0 1 3 10.46S2.79 8.3 5.28 6.19c0 0 2.82-2.61 1.84-4.54L7.83 1a6.57 6.57 0 0 1 2.61 6.94 2.57 2.57 0 0 0 .56-.81l.87-.07c.07.12 1.84 2.93.89 5.3A4.72 4.72 0 0 1 9.13 15zm-2-6.95l.87.39a3 3 0 0 0 .92 2.48 2.64 2.64 0 0 1 1 2.8A3.241 3.241 0 0 0 11.8 12a4.87 4.87 0 0 0-.41-3.63 1.85 1.85 0 0 1-1.84.86l-.35-.68a5.31 5.31 0 0 0-.89-5.8C8.17 4.87 6 6.83 5.93 6.94 3.86 8.7 4 10.33 4 10.4a3.47 3.47 0 0 0 1.59 3.14C5 12.14 5 10.46 7.16 8.05h-.03z"
        /> < title > { title } < / title > < / svg >
    }
}
