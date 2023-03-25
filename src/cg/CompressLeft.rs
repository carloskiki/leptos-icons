#[cfg(feature = "CgCompressLeft")]
use leptos::*;
#[cfg(feature = "CgCompressLeft")]
///This icon requires the feature `CgCompressLeft` to be enabled.
#[component]
pub fn CompressLeft(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M7.97867 9.45703L4.40883 9.45423L4.40726 11.4542L11.4073 11.4597L11.4127 4.45972L9.41274 4.45815L9.40992 8.05978L3.09616 1.76935L1.68457 3.18618L7.97867 9.45703Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M19.5615 14.5521L19.5535 12.5521L12.5536 12.58L12.5814 19.5799L14.5814 19.572L14.5671 15.9706L20.9105 22.2307L22.3153 20.8071L15.9914 14.5663L19.5615 14.5521Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
