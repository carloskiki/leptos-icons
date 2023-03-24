#[cfg(feature = "OcLgSmiley")]
use leptos::*;
#[cfg(feature = "OcLgSmiley")]
///This icon requires the feature `OcLgSmiley` to be enabled.
#[component]
pub fn Smiley(
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
        "M8.456 14.494a.75.75 0 0 1 1.068.17 3.08 3.08 0 0 0 .572.492A3.381 3.381 0 0 0 12 15.72c.855 0 1.487-.283 1.904-.562a3.081 3.081 0 0 0 .572-.492l.021-.026a.75.75 0 0 1 1.197.905l-.027.034c-.013.016-.03.038-.052.063-.044.05-.105.119-.184.198a4.569 4.569 0 0 1-.695.566A4.88 4.88 0 0 1 12 17.22a4.88 4.88 0 0 1-2.736-.814 4.57 4.57 0 0 1-.695-.566 3.253 3.253 0 0 1-.236-.261c-.259-.332-.223-.824.123-1.084Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 1c6.075 0 11 4.925 11 11s-4.925 11-11 11S1 18.075 1 12 5.925 1 12 1ZM2.5 12a9.5 9.5 0 0 0 9.5 9.5 9.5 9.5 0 0 0 9.5-9.5A9.5 9.5 0 0 0 12 2.5 9.5 9.5 0 0 0 2.5 12Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M9 10.75a1.25 1.25 0 1 1-2.5 0 1.25 1.25 0 0 1 2.5 0ZM16.25 12a1.25 1.25 0 1 0 0-2.5 1.25 1.25 0 0 0 0 2.5Z"
        /> < title > { title } < / title > < / svg >
    }
}
