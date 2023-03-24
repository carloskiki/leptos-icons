#[cfg(feature = "TiPointOfInterest")]
use leptos::*;
#[cfg(feature = "TiPointOfInterest")]
///This icon requires the feature `TiPointOfInterest` to be enabled.
#[component]
pub fn PointOfInterest(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M16.5 11c1.93 0 3.5-1.57 3.5-3.5s-1.57-3.5-3.5-3.5-3.5 1.57-3.5 3.5v1.5h-2v-1.5c0-1.93-1.57-3.5-3.5-3.5s-3.5 1.57-3.5 3.5 1.57 3.5 3.5 3.5h1.5v2h-1.5c-1.93 0-3.5 1.57-3.5 3.5s1.57 3.5 3.5 3.5 3.5-1.57 3.5-3.5v-1.5h2v1.5c0 1.93 1.57 3.5 3.5 3.5s3.5-1.57 3.5-3.5-1.57-3.5-3.5-3.5h-1.5v-2h1.5zm-1.5-3.5c0-.828.673-1.5 1.5-1.5s1.5.672 1.5 1.5c0 .826-.673 1.5-1.5 1.5h-1.5v-1.5zm-6 9c0 .826-.673 1.5-1.5 1.5s-1.5-.674-1.5-1.5c0-.828.673-1.5 1.5-1.5h1.5v1.5zm0-7.5h-1.5c-.827 0-1.5-.674-1.5-1.5 0-.828.673-1.5 1.5-1.5s1.5.672 1.5 1.5v1.5zm4 4h-2v-2h2v2zm3.5 2c.827 0 1.5.672 1.5 1.5 0 .826-.673 1.5-1.5 1.5s-1.5-.674-1.5-1.5v-1.5h1.5z"
        /> < title > { title } < / title > < / svg >
    }
}
