#[cfg(feature = "SiJfrogbintray")]
use leptos::*;
#[cfg(feature = "SiJfrogbintray")]
///This icon requires the feature `SiJfrogbintray` to be enabled.
#[component]
pub fn Jfrogbintray(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M2.617 22.316h18.766V24H2.617zm15.88-12.632l-5.655 5.655V3.249l1.744 1.743L15.79 3.79 12 0 8.21 3.79l1.204 1.203 1.744-1.804v12.15L5.504 9.686H7.97V8H2.617v5.354H4.3v-2.527L12 18.526l7.698-7.699v2.527h1.685V8H16.03v1.684z"
        /> < title > { title } < / title > < / svg >
    }
}
