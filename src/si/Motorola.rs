#[cfg(feature = "SiMotorola")]
use leptos::*;
#[cfg(feature = "SiMotorola")]
///This icon requires the feature `SiMotorola` to be enabled.
#[component]
pub fn Motorola(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M12 0C5.373 0 0 5.373 0 12s5.373 12 12 12 12-5.373 12-12C24.002 5.375 18.632.002 12.007 0H12zm7.327 18.065s-.581-2.627-1.528-4.197c-.514-.857-1.308-1.553-2.368-1.532-.745 0-1.399.423-2.2 1.553-.469.77-.882 1.573-1.235 2.403 0 0-.29-.675-.63-1.343a8.038 8.038 0 0 0-.605-1.049c-.804-1.13-1.455-1.539-2.2-1.553-1.049-.021-1.854.675-2.364 1.528-.948 1.574-1.528 4.197-1.528 4.197h-.864l4.606-15.12 3.56 11.804.024.021.024-.021 3.56-11.804 4.61 15.113h-.862z"
        /> < title > { title } < / title > < / svg >
    }
}
