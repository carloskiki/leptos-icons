#[cfg(feature = "SiLetterboxd")]
use leptos::*;
#[cfg(feature = "SiLetterboxd")]
///This icon requires the feature `SiLetterboxd` to be enabled.
#[component]
pub fn Letterboxd(
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
        "M8.29 16.752V7.2H6.546V4.8h6.328v2.4h-1.746v9.574h3.925v-2.618h2.839V19.2H6.545v-2.448h1.746zM0 12c0 6.628 5.372 12 12 12s12-5.372 12-12S18.628 0 12 0 0 5.372 0 12z"
        /> < title > { title } < / title > < / svg >
    }
}
