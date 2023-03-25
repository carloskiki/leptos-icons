#[cfg(feature = "IoPush")]
use leptos::*;
#[cfg(feature = "IoPush")]
///This icon requires the feature `IoPush` to be enabled.
#[component]
pub fn Push(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M376,352H272V198.63l52.69,52.68a16,16,0,0,0,22.62-22.62l-80-80a16,16,0,0,0-22.62,0l-80,80a16,16,0,0,0,22.62,22.62L240,198.63V352H136a56.06,56.06,0,0,1-56-56V88a56.06,56.06,0,0,1,56-56H376a56.06,56.06,0,0,1,56,56V296A56.06,56.06,0,0,1,376,352Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M272,464a16,16,0,0,1-32,0V352h32Z" /> < title > { title } < / title > < / svg >
    }
}
