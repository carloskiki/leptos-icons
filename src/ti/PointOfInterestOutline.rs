#[cfg(feature = "TiPointOfInterestOutline")]
use leptos::*;
#[cfg(feature = "TiPointOfInterestOutline")]
///This icon requires the feature `TiPointOfInterestOutline` to be enabled.
#[component]
pub fn PointOfInterestOutline(
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
        "M16.5 4c1.93 0 3.5 1.57 3.5 3.5s-1.57 3.5-3.5 3.5h-1.5v2h1.5c1.93 0 3.5 1.57 3.5 3.5s-1.57 3.5-3.5 3.5-3.5-1.57-3.5-3.5v-1.5h-2v1.5c0 1.93-1.57 3.5-3.5 3.5s-3.5-1.57-3.5-3.5 1.57-3.5 3.5-3.5h1.5v-2h-1.5c-1.93 0-3.5-1.57-3.5-3.5s1.57-3.5 3.5-3.5 3.5 1.57 3.5 3.5v1.5h2v-1.5c0-1.93 1.57-3.5 3.5-3.5m-1.5 5h1.5c.827 0 1.5-.674 1.5-1.5 0-.828-.673-1.5-1.5-1.5s-1.5.672-1.5 1.5v1.5m-7.5 0h1.5v-1.5c0-.828-.673-1.5-1.5-1.5s-1.5.672-1.5 1.5c0 .826.673 1.5 1.5 1.5m9 9c.827 0 1.5-.674 1.5-1.5 0-.828-.673-1.5-1.5-1.5h-1.5v1.5c0 .826.673 1.5 1.5 1.5m-9 0c.827 0 1.5-.674 1.5-1.5v-1.5h-1.5c-.827 0-1.5.672-1.5 1.5 0 .826.673 1.5 1.5 1.5m9-16c-1.857 0-3.504.926-4.5 2.341-.996-1.415-2.643-2.341-4.5-2.341-3.033 0-5.5 2.468-5.5 5.5 0 1.857.926 3.504 2.341 4.5-1.415.996-2.341 2.643-2.341 4.5 0 3.032 2.467 5.5 5.5 5.5 1.857 0 3.504-.926 4.5-2.341.996 1.415 2.643 2.341 4.5 2.341 3.033 0 5.5-2.468 5.5-5.5 0-1.857-.926-3.504-2.341-4.5 1.415-.996 2.341-2.643 2.341-4.5 0-3.032-2.467-5.5-5.5-5.5zM13 11h-2v2h2z"
        /> < title > { title } < / title > < / svg >
    }
}
