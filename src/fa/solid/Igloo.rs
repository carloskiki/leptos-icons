#[cfg(feature = "FaSolidIgloo")]
use leptos::*;
#[cfg(feature = "FaSolidIgloo")]
///This icon requires the feature `FaSolidIgloo` to be enabled.
#[component]
pub fn Igloo(
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
        stroke_witdh = "0" style = style viewBox = "0 0 576 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M320 33.8V160H48.5C100.2 82.8 188.1 32 288 32c10.8 0 21.5 .6 32 1.8zM352 160V39.1C424.9 55.7 487.2 99.8 527.5 160H352zM29.9 192H96V320H0c0-46 10.8-89.4 29.9-128zM192 320H128V192H448V320H384v32H576v80c0 26.5-21.5 48-48 48H352V352c0-35.3-28.7-64-64-64s-64 28.7-64 64V480H48c-26.5 0-48-21.5-48-48V352H192V320zm288 0V192h66.1c19.2 38.6 29.9 82 29.9 128H480z"
        /> < title > { title } < / title > < / svg >
    }
}
