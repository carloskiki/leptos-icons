#[cfg(feature = "BiLogosGoogleCloud")]
use leptos::*;
#[cfg(feature = "BiLogosGoogleCloud")]
///This icon requires the feature `BiLogosGoogleCloud` to be enabled.
#[component]
pub fn GoogleCloud(
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
        "M19.511 9.722a7.833 7.833 0 0 0-2.359-3.804l-.035.035.005-.042A7.81 7.81 0 0 0 4.418 9.722c.031-.013.066-.013.099-.023a5.643 5.643 0 0 0-.306 9.166l.006-.006-.006.024a5.612 5.612 0 0 0 3.407 1.134h4.321l.024.024h4.341a5.644 5.644 0 0 0 3.207-10.319zm-3.206 6.845h-4.341l-.006.006v-.031h-4.34c-.308 0-.611-.066-.892-.193l.002-.001a2.17 2.17 0 1 1 2.87-2.871l2.518-2.518a5.634 5.634 0 0 0-3.396-2.1c.018-.009.035-.024.05-.021a4.334 4.334 0 0 1 5.931-.451h.046a4.334 4.334 0 0 1 1.558 3.407v.433a2.17 2.17 0 1 1 0 4.34z"
        /> < title > { title } < / title > < / svg >
    }
}
