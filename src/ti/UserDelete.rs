#[cfg(feature = "TiUserDelete")]
use leptos::*;
#[cfg(feature = "TiUserDelete")]
///This icon requires the feature `TiUserDelete` to be enabled.
#[component]
pub fn UserDelete(
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
        "M21 14h-6c-.553 0-1-.447-1-1s.447-1 1-1h6c.553 0 1 .447 1 1s-.447 1-1 1zM14 9c0 1.381-.56 2.631-1.464 3.535-.905.905-2.155 1.465-3.536 1.465s-2.631-.56-3.536-1.465c-.904-.904-1.464-2.154-1.464-3.535s.56-2.631 1.464-3.535c.905-.905 2.155-1.465 3.536-1.465s2.631.56 3.536 1.465c.904.904 1.464 2.154 1.464 3.535zM9 15c-3.75 0-6 2-6 4 0 1 2.25 2 6 2 3.518 0 6-1 6-2 0-2-2.354-4-6-4z"
        /> < title > { title } < / title > < / svg >
    }
}
