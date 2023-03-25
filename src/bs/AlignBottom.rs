#[cfg(feature = "BsAlignBottom")]
use leptos::*;
#[cfg(feature = "BsAlignBottom")]
///This icon requires the feature `BsAlignBottom` to be enabled.
#[component]
pub fn AlignBottom(
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
        stroke_witdh = "0" style = style width = "16" height = "16" fill = "currentColor"
        class = "bi bi-align-bottom" viewBox = "0 0 16 16" width = size.clone() height =
        size xmlns = "http://www.w3.org/2000/svg" > < rect xmlns =
        "http://www.w3.org/2000/svg" width = "4" height = "12" x = "6" y = "1" rx = "1"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M1.5 14a.5.5 0 0 0 0 1v-1zm13 1a.5.5 0 0 0 0-1v1zm-13 0h13v-1h-13v1z" /> < title
        > { title } < / title > < / svg >
    }
}
