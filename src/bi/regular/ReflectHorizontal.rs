#[cfg(feature = "BiRegularReflectHorizontal")]
use leptos::*;
#[cfg(feature = "BiRegularReflectHorizontal")]
///This icon requires the feature `BiRegularReflectHorizontal` to be enabled.
#[component]
pub fn ReflectHorizontal(
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
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M6 21h12l-6-6-6 6zM18 3H6l6 6 6-6zM3 11h3v2H3zm5 0h3v2H8zm5 0h3v2h-3zm5 0h3v2h-3z"
        /> < title > { title } < / title > < / svg >
    }
}
