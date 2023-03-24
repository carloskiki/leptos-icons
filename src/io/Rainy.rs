#[cfg(feature = "IoRainy")]
use leptos::*;
#[cfg(feature = "IoRainy")]
///This icon requires the feature `IoRainy` to be enabled.
#[component]
pub fn Rainy(
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
        "0 0 512 512" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M456.26,139.37c-16.77-16.73-39.17-28.41-65.17-34a16,16,0,0,1-11.19-9,142.24,142.24,0,0,0-42.19-53.21C314.48,25.39,286.23,16,256,16a140.24,140.24,0,0,0-93.5,35.32c-24.2,21.56-40.91,51.34-48.43,85.83a16.05,16.05,0,0,1-11.72,12.18c-25,6.3-35.71,12.54-49.21,24.56C34,190.93,24,214.14,24,240.8c0,30.55,11.23,55.64,32.47,72.56C75.08,328.17,100.5,336,130,336H364c33.2,0,64.11-11.46,87-32.28,23.84-21.65,37-51.67,37-84.52C488,187.71,477,160.11,456.26,139.37Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M112,448a16,16,0,0,1-13.3-24.88l32-48a16,16,0,0,1,26.62,17.76l-32,48A16,16,0,0,1,112,448Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M160,496a16,16,0,0,1-13.29-24.88l64-96a16,16,0,0,1,26.62,17.76l-64,96A16,16,0,0,1,160,496Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M272,448a16,16,0,0,1-13.3-24.88l32-48a16,16,0,0,1,26.62,17.76l-32,48A16,16,0,0,1,272,448Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M320,496a16,16,0,0,1-13.3-24.88l64-96a16,16,0,0,1,26.62,17.76l-64,96A16,16,0,0,1,320,496Z"
        /> < title > { title } < / title > < / svg >
    }
}
