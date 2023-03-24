#[cfg(feature = "RiOthersFillSeedling")]
use leptos::*;
#[cfg(feature = "RiOthersFillSeedling")]
///This icon requires the feature `RiOthersFillSeedling` to be enabled.
#[component]
pub fn Seedling(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0H24V24H0z" />< path d
        =
        "M22 7v2.5c0 3.59-2.91 6.5-6.5 6.5H13v5h-2v-7l.019-1c.255-3.356 3.06-6 6.481-6H22zM6 3c3.092 0 5.716 2.005 6.643 4.786-1.5 1.275-2.49 3.128-2.627 5.214H9c-3.866 0-7-3.134-7-7V3h4z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
