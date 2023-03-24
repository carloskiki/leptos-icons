#[cfg(feature = "BiSolidCloudRain")]
use leptos::*;
#[cfg(feature = "BiSolidCloudRain")]
///This icon requires the feature `BiSolidCloudRain` to be enabled.
#[component]
pub fn CloudRain(
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
    let style = format!("{} color: {};", style, color);
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M18.944 10.112C18.507 6.67 15.56 4 12 4 9.244 4 6.85 5.611 5.757 8.15 3.609 8.792 2 10.82 2 13c0 2.757 2.243 5 5 5h1v3h2v-3h4v3h2v-3h2c2.206 0 4-1.794 4-4a4.01 4.01 0 0 0-3.056-3.888z"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M11 19h2v3h-2z" /> < title > {
        title } < / title > < / svg >
    }
}
