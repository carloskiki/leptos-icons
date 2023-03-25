#[cfg(feature = "FaSolidGuaraniSign")]
use leptos::*;
#[cfg(feature = "FaSolidGuaraniSign")]
///This icon requires the feature `FaSolidGuaraniSign` to be enabled.
#[component]
pub fn GuaraniSign(
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
        stroke_witdh = "0" style = style viewBox = "0 0 384 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M192 0c-17.7 0-32 14.3-32 32V66.7C69.2 81.9 0 160.9 0 256s69.2 174.1 160 189.3V480c0 17.7 14.3 32 32 32s32-14.3 32-32V445.3c90.8-15.2 160-94.2 160-189.3c0-17.7-14.3-32-32-32H224V132c22.1 5.7 41.8 17.1 57.6 32.6c12.6 12.4 32.9 12.2 45.3-.4s12.2-32.9-.5-45.3C299 92 263.5 73.3 224 66.7V32c0-17.7-14.3-32-32-32zM160 132V380c-55.2-14.2-96-64.3-96-124s40.8-109.8 96-124zM224 380V288h92c-11.6 45-47 80.4-92 92z"
        /> < title > { title } < / title > < / svg >
    }
}
