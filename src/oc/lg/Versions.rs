#[cfg(feature = "OcLgVersions")]
use leptos::*;
#[cfg(feature = "OcLgVersions")]
///This icon requires the feature `OcLgVersions` to be enabled.
#[component]
pub fn Versions(
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
        "M10 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h11a2 2 0 0 1 2 2v16a2 2 0 0 1-2 2Zm-.5-2a.5.5 0 0 0 .5.5h11a.5.5 0 0 0 .5-.5V4a.5.5 0 0 0-.5-.5H10a.5.5 0 0 0-.5.5ZM6.17 4.165a.75.75 0 0 1-.335 1.006c-.228.114-.295.177-.315.201a.035.035 0 0 0-.008.016.423.423 0 0 0-.012.112v13c0 .07.008.102.012.112a.03.03 0 0 0 .008.016c.02.024.087.087.315.201a.749.749 0 1 1-.67 1.342c-.272-.136-.58-.315-.81-.598C4.1 19.259 4 18.893 4 18.5v-13c0-.393.1-.759.355-1.073.23-.283.538-.462.81-.598a.75.75 0 0 1 1.006.336ZM2.15 5.624a.75.75 0 0 1-.274 1.025c-.15.087-.257.17-.32.245C1.5 6.96 1.5 6.99 1.5 7v10c0 .01 0 .04.056.106.063.074.17.158.32.245a.75.75 0 0 1-.752 1.298C.73 18.421 0 17.907 0 17V7c0-.907.73-1.42 1.124-1.65a.75.75 0 0 1 1.025.274Z"
        /> < title > { title } < / title > < / svg >
    }
}
