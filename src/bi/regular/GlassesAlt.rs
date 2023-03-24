#[cfg(feature = "BiRegularGlassesAlt")]
use leptos::*;
#[cfg(feature = "BiRegularGlassesAlt")]
///This icon requires the feature `BiRegularGlassesAlt` to be enabled.
#[component]
pub fn GlassesAlt(
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
        "m21.948 11.684-2-6A.997.997 0 0 0 19 5h-3v2h2.279l1.334 4H15c-1.103 0-2 .897-2 2h-2c0-1.103-.897-2-2-2H4.387l1.334-4H8V5H5a.998.998 0 0 0-.948.684l-2 6 .012.004A.928.928 0 0 0 2 12v4c0 1.654 1.346 3 3 3h3c1.654 0 3-1.346 3-3v-1h2v1c0 1.654 1.346 3 3 3h3c1.654 0 3-1.346 3-3v-4a.964.964 0 0 0-.063-.313l.011-.003zM9 16c0 .551-.448 1-1 1H5c-.552 0-1-.449-1-1v-3h5v3zm11 0c0 .551-.448 1-1 1h-3c-.552 0-1-.449-1-1v-3h5v3z"
        /> < title > { title } < / title > < / svg >
    }
}
