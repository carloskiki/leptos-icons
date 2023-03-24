#[cfg(feature = "IoInvertMode")]
use leptos::*;
#[cfg(feature = "IoInvertMode")]
///This icon requires the feature `IoInvertMode` to be enabled.
#[component]
pub fn InvertMode(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < circle xmlns =
        "http://www.w3.org/2000/svg" fill = "none" stroke = "#000" stroke - miterlimit =
        "10" stroke - width = "32" cx = "256" cy = "256" r = "208" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M256,176V336a80,80,0,0,0,0-160Z" />< path xmlns
        = "http://www.w3.org/2000/svg" d =
        "M256,48V176a80,80,0,0,0,0,160V464C141.12,464,48,370.88,48,256S141.12,48,256,48Z"
        /> < title > { title } < / title > < / svg >
    }
}
