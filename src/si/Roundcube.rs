#[cfg(feature = "SiRoundcube")]
use leptos::*;
#[cfg(feature = "SiRoundcube")]
///This icon requires the feature `SiRoundcube` to be enabled.
#[component]
pub fn Roundcube(
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
        "M12.002.072a8.302 8.302 0 0 0-8.266 7.512L.498 9.454l4.682 2.704A7.8 7.8 0 0 1 12.002.572a7.802 7.802 0 0 1 6.824 11.582l4.676-2.7-3.236-1.87A8.302 8.302 0 0 0 12.002.072zM0 9.742v7.399l11.75 6.787v-7.399L0 9.742zm24 0l-5.777 3.338-5.248 3.031h-.002l-.108.063-.615.355v7.399L24 17.14V9.744z"
        /> < title > { title } < / title > < / svg >
    }
}
