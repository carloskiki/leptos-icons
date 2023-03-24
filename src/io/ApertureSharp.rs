#[cfg(feature = "IoApertureSharp")]
use leptos::*;
#[cfg(feature = "IoApertureSharp")]
///This icon requires the feature `IoApertureSharp` to be enabled.
#[component]
pub fn ApertureSharp(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = { size.clone() } height = { size } > < polygon xmlns =
        "http://www.w3.org/2000/svg" points = "216 48 345.49 176.18 345.49 48 216 48" /><
        polygon xmlns = "http://www.w3.org/2000/svg" points =
        "181.47 58.38 80 134 256 134 181.47 58.38" />< polygon xmlns =
        "http://www.w3.org/2000/svg" points = "336 344 464 344 464 216 336 344" /><
        polygon xmlns = "http://www.w3.org/2000/svg" points =
        "454 182 378 80 378 256 454 182" />< polygon xmlns = "http://www.w3.org/2000/svg"
        points = "48 166 48 294 176 166 48 166" />< polygon xmlns =
        "http://www.w3.org/2000/svg" points = "330 454 432 378 256 378 330 454" /><
        polygon xmlns = "http://www.w3.org/2000/svg" points =
        "58 330 134 432 134 256 58 330" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M345.49,222.12l-55.55-55.46H222.06l-55.55,55.46v67.76l55.62,55.52c.44,0,.88-.06,1.33-.06h66.48l55.55-55.46Z"
        />< polygon xmlns = "http://www.w3.org/2000/svg" points =
        "165.98 336.09 166 464 294 464 165.98 336.09" /> < title > { title } < / title >
        < / svg >
    }
}
