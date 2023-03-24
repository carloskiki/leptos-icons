#[cfg(feature = "HiLgSolidStop")]
use leptos::*;
#[cfg(feature = "HiLgSolidStop")]
///This icon requires the feature `HiLgSolidStop` to be enabled.
#[component]
pub fn Stop(
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
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M4.5 7.5C4.5 5.84315 5.84315 4.5 7.5 4.5H16.5C18.1569 4.5 19.5 5.84315 19.5 7.5V16.5C19.5 18.1569 18.1569 19.5 16.5 19.5H7.5C5.84315 19.5 4.5 18.1569 4.5 16.5V7.5Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
