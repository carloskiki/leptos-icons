#[cfg(feature = "SiSpeakerdeck")]
use leptos::*;
#[cfg(feature = "SiSpeakerdeck")]
///This icon requires the feature `SiSpeakerdeck` to be enabled.
#[component]
pub fn Speakerdeck(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M10.025 13.875H4.687a4.688 4.688 0 0 1 0-9.375h6.227a1.875 1.875 0 0 1 0 3.75H4.592a.937.937 0 1 0 0 1.875h5.337a4.687 4.687 0 1 1 0 9.375H1.875a1.875 1.875 0 0 1 0-3.75h8.15a.938.938 0 0 0 0-1.875zM13.97 19.5a5.635 5.635 0 0 0 2.396-3.75h3.026a.93.93 0 0 0 .921-.938V9.189a.93.93 0 0 0-.921-.938h-5.497c.438-.498.704-1.155.704-1.875s-.266-1.377-.704-1.875h6.418C22.35 4.5 24 6.179 24 8.25v7.5c0 2.071-1.65 3.75-3.687 3.75H13.97z"
        /> < title > { title } < / title > < / svg >
    }
}
