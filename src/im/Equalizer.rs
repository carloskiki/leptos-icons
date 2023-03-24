#[cfg(feature = "ImEqualizer")]
use leptos::*;
#[cfg(feature = "ImEqualizer")]
///This icon requires the feature `ImEqualizer` to be enabled.
#[component]
pub fn Equalizer(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" xmlns : xlink = "http://www.w3.org/1999/xlink" fill
        = "#000000" d =
        "M7 2v-0.25c0-0.413-0.338-0.75-0.75-0.75h-2.5c-0.413 0-0.75 0.337-0.75 0.75v0.25h-3v2h3v0.25c0 0.412 0.337 0.75 0.75 0.75h2.5c0.412 0 0.75-0.338 0.75-0.75v-0.25h9v-2h-9zM4 4v-2h2v2h-2zM13 6.75c0-0.412-0.338-0.75-0.75-0.75h-2.5c-0.412 0-0.75 0.338-0.75 0.75v0.25h-9v2h9v0.25c0 0.412 0.338 0.75 0.75 0.75h2.5c0.412 0 0.75-0.338 0.75-0.75v-0.25h3v-2h-3v-0.25zM10 9v-2h2v2h-2zM7 11.75c0-0.412-0.338-0.75-0.75-0.75h-2.5c-0.413 0-0.75 0.338-0.75 0.75v0.25h-3v2h3v0.25c0 0.412 0.337 0.75 0.75 0.75h2.5c0.412 0 0.75-0.338 0.75-0.75v-0.25h9v-2h-9v-0.25zM4 14v-2h2v2h-2z"
        /> < title > { title } < / title > < / svg >
    }
}
