#[cfg(feature = "BiRegularCard")]
use leptos::*;
#[cfg(feature = "BiRegularCard")]
///This icon requires the feature `BiRegularCard` to be enabled.
#[component]
pub fn Card(
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
        "M17.999 17c1.103 0 2-.897 2-2V5c0-1.103-.897-2-2-2h-12c-1.103 0-2 .897-2 2v10c0 1.103.897 2 2 2h12zm-12-12h12l.002 10H5.999V5zm-2 14h16v2h-16z"
        /> < title > { title } < / title > < / svg >
    }
}
