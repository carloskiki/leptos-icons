#[cfg(feature = "IoDiscSharp")]
use leptos::*;
#[cfg(feature = "IoDiscSharp")]
///This icon requires the feature `IoDiscSharp` to be enabled.
#[component]
pub fn DiscSharp(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < circle xmlns = "http://www.w3.org/2000/svg" cx =
        "256" cy = "256" r = "32" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M414.39,97.61A224,224,0,1,0,97.61,414.39,224,224,0,1,0,414.39,97.61ZM256,336a80,80,0,1,1,80-80A80.09,80.09,0,0,1,256,336Z"
        /> < title > { title } < / title > < / svg >
    }
}
