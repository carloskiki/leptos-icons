#[cfg(feature = "RiMapLineCarWashing")]
use leptos::*;
#[cfg(feature = "RiMapLineCarWashing")]
///This icon requires the feature `RiMapLineCarWashing` to be enabled.
#[component]
pub fn CarWashing(
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
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M19 21H5v1a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V12l2.417-4.029A2 2 0 0 1 6.132 7h11.736a2 2 0 0 1 1.715.971L22 12v10a1 1 0 0 1-1 1h-1a1 1 0 0 1-1-1v-1zm1-7H4v5h16v-5zM4.332 12h15.336l-1.8-3H6.132l-1.8 3zM5.44 3.44L6.5 2.378l1.06 1.06a1.5 1.5 0 1 1-2.121 0zm5.5 0L12 2.378l1.06 1.06a1.5 1.5 0 1 1-2.121 0zm5.5 0L17.5 2.378l1.06 1.06a1.5 1.5 0 1 1-2.121 0zM6.5 18a1.5 1.5 0 1 1 0-3 1.5 1.5 0 0 1 0 3zm11 0a1.5 1.5 0 1 1 0-3 1.5 1.5 0 0 1 0 3z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
