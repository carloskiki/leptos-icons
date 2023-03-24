#[cfg(feature = "TiTime")]
use leptos::*;
#[cfg(feature = "TiTime")]
///This icon requires the feature `TiTime` to be enabled.
#[component]
pub fn Time(
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
        "M16 13c0-.55-.45-1-1-1h-3c-.55 0-1 .45-1 1s.45 1 1 1h3c.55 0 1-.45 1-1zM12 6c3.859 0 7 3.141 7 7s-3.141 7-7 7-7-3.141-7-7 3.141-7 7-7m0-2c-4.971 0-9 4.029-9 9s4.029 9 9 9 9-4.029 9-9-4.029-9-9-9zM13 10c0-.55-.45-1-1-1s-1 .45-1 1v3c0 .55.45 1 1 1s1-.45 1-1v-3zM12 8c2.757 0 5 2.243 5 5s-2.243 5-5 5-5-2.243-5-5 2.243-5 5-5m0-1c-3.312 0-6 2.686-6 6 0 3.312 2.688 6 6 6s6-2.688 6-6c0-3.314-2.688-6-6-6z"
        /> < title > { title } < / title > < / svg >
    }
}
