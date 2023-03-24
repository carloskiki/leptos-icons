#[cfg(feature = "RiLogosFillHonorOfKings")]
use leptos::*;
#[cfg(feature = "RiLogosFillHonorOfKings")]
///This icon requires the feature `RiLogosFillHonorOfKings` to be enabled.
#[component]
pub fn HonorOfKings(
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
        "M21.158 4.258c.034 3.5.591 4.811.788 6.701.301 2.894-.657 5.894-2.875 8.112-3.666 3.666-9.471 3.89-13.4.673l2.852-2.853c2.344 1.67 5.617 1.454 7.72-.648 2.102-2.103 2.318-5.377.648-7.72l4.267-4.265zm-2.83-.002l-2.851 2.853c-2.344-1.67-5.617-1.454-7.72.648-2.102 2.103-2.318 5.376-.648 7.72l-4.267 4.265c-.034-3.5-.591-4.811-.788-6.701-.301-2.894.657-5.894 2.875-8.112 3.666-3.666 9.471-3.89 13.4-.673zM12 8c2.21 0 4 1.79 4 4s-1.79 4-4 4-4-1.79-4-4 1.79-4 4-4zm0 2.5c-.828 0-1.5.672-1.5 1.5s.672 1.5 1.5 1.5 1.5-.672 1.5-1.5-.672-1.5-1.5-1.5z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
