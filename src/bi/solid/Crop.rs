#[cfg(feature = "BiSolidCrop")]
use leptos::*;
#[cfg(feature = "BiSolidCrop")]
///This icon requires the feature `BiSolidCrop` to be enabled.
#[component]
pub fn Crop(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M19 7.5C19 6.121 17.879 5 16.5 5H8V2H5v3H2v3h14v14h3v-3h3v-3h-3V7.5z" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M8 10H5v6.5C5 17.879 6.121 19 7.5 19H14v-3H8v-6z" /> < title > { title } < /
        title > < / svg >
    }
}
