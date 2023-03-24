#[cfg(feature = "RiLogosFillMedium")]
use leptos::*;
#[cfg(feature = "RiLogosFillMedium")]
///This icon requires the feature `RiLogosFillMedium` to be enabled.
#[component]
pub fn Medium(
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
        "M4 3h16a1 1 0 0 1 1 1v16a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1zm13.3 12.94c-.1-.05-.15-.2-.15-.301V8.006c0-.1.05-.25.15-.351l.955-1.105V6.5H14.84l-2.56 6.478L9.366 6.5H5.852v.05l.903 1.256c.201.2.251.502.251.753v5.523c.05.302 0 .653-.15.954L5.5 16.894v.05h3.616v-.05L7.76 15.087c-.15-.302-.201-.603-.15-.954V9.11c.05.1.1.1.15.301l3.414 7.633h.05L14.54 8.76c-.05.3-.05.652-.05.904v5.925c0 .15-.05.25-.15.351l-1.005.954v.05h4.921v-.05l-.954-.954z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
