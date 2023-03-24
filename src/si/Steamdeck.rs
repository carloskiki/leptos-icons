#[cfg(feature = "SiSteamdeck")]
use leptos::*;
#[cfg(feature = "SiSteamdeck")]
///This icon requires the feature `SiSteamdeck` to be enabled.
#[component]
pub fn Steamdeck(
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
        "M8.494 0v4.309c4.242 0 7.694 3.45 7.694 7.691s-3.452 7.691-7.694 7.691V24c6.617 0 12-5.383 12-12s-5.383-12-12-12zm10.819 3.62v.194h.328v.893h.228v-.893h.33V3.62zm1.037 0v1.087h.207v-.684l.298.653h.14l.29-.66v.691h.219V3.619h-.23l-.338.772-.368-.772zM8.494 6.011a5.998 5.998 0 0 0-5.998 6 5.998 5.998 0 1 0 11.998 0 6 6 0 0 0-6-6z"
        /> < title > { title } < / title > < / svg >
    }
}
