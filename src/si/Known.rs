#[cfg(feature = "SiKnown")]
use leptos::*;
#[cfg(feature = "SiKnown")]
///This icon requires the feature `SiKnown` to be enabled.
#[component]
pub fn Known(
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
        "M18.387 16.926h.604v1.936h-6.086v-1.936h.904s.333-.072.26-.386l-2.392-3.776-1.893 1.847v1.322c0 .653.324.993.687.993h.844v1.936H5.414v-1.936h.741c.364 0 .688-.34.688-.993V7.992c0-.364-.324-.855-.688-.855h-.741V5.201h5.901v1.936h-.844c-.363 0-.687.491-.687.855v3.83l4.087-4.144a.316.316 0 0 0-.219-.541h-.747V5.201H19v1.936h-.872c-.363 0-.867.176-1.225.525l-3.058 2.985 3.396 5.276c.304.434.772 1.003 1.146 1.003zM24 12c0 6.627-5.373 12-12 12S0 18.627 0 12 5.373 0 12 0s12 5.373 12 12zm-1.684 0c0-5.697-4.619-10.316-10.316-10.316C6.303 1.684 1.684 6.303 1.684 12c0 5.697 4.619 10.316 10.316 10.316 5.697 0 10.316-4.619 10.316-10.316z"
        /> < title > { title } < / title > < / svg >
    }
}
