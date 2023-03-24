#[cfg(feature = "OcLgShield")]
use leptos::*;
#[cfg(feature = "OcLgShield")]
///This icon requires the feature `OcLgShield` to be enabled.
#[component]
pub fn Shield(
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
        "M13 15.5a1 1 0 1 1-2 0 1 1 0 0 1 2 0Zm-.25-8.25a.75.75 0 0 0-1.5 0v4.5a.75.75 0 0 0 1.5 0v-4.5Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M11.46.637a1.748 1.748 0 0 1 1.08 0l8.25 2.675A1.75 1.75 0 0 1 22 4.976V10c0 6.19-3.77 10.705-9.401 12.83a1.704 1.704 0 0 1-1.198 0C5.771 20.704 2 16.19 2 10V4.976c0-.76.49-1.43 1.21-1.664Zm.617 1.426a.253.253 0 0 0-.154 0L3.673 4.74a.25.25 0 0 0-.173.237V10c0 5.461 3.28 9.483 8.43 11.426a.199.199 0 0 0 .14 0C17.22 19.483 20.5 15.46 20.5 10V4.976a.25.25 0 0 0-.173-.237Z"
        /> < title > { title } < / title > < / svg >
    }
}
