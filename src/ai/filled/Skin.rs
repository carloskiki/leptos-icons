#[cfg(feature = "AiFilledSkin")]
use leptos::*;
#[cfg(feature = "AiFilledSkin")]
///This icon requires the feature `AiFilledSkin` to be enabled.
#[component]
pub fn Skin(
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
        stroke_witdh = "0" style = style class = "icon" viewBox = "0 0 1024 1024" width =
        size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M870 126H663.8c-17.4 0-32.9 11.9-37 29.3C614.3 208.1 567 246 512 246s-102.3-37.9-114.8-90.7a37.93 37.93 0 0 0-37-29.3H154a44 44 0 0 0-44 44v252a44 44 0 0 0 44 44h75v388a44 44 0 0 0 44 44h478a44 44 0 0 0 44-44V466h75a44 44 0 0 0 44-44V170a44 44 0 0 0-44-44z"
        /> < title > { title } < / title > < / svg >
    }
}
