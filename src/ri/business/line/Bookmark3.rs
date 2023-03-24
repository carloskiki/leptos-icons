#[cfg(feature = "RiBusinessLineBookmark3")]
use leptos::*;
#[cfg(feature = "RiBusinessLineBookmark3")]
///This icon requires the feature `RiBusinessLineBookmark3` to be enabled.
#[component]
pub fn Bookmark3(
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
        "M4 2h16a1 1 0 0 1 1 1v19.276a.5.5 0 0 1-.704.457L12 19.03l-8.296 3.702A.5.5 0 0 1 3 22.276V3a1 1 0 0 1 1-1zm15 17.965V4H5v15.965l7-3.124 7 3.124zM12 13.5l-2.939 1.545.561-3.272-2.377-2.318 3.286-.478L12 6l1.47 2.977 3.285.478-2.377 2.318.56 3.272L12 13.5z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
