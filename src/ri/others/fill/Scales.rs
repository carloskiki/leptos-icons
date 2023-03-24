#[cfg(feature = "RiOthersFillScales")]
use leptos::*;
#[cfg(feature = "RiOthersFillScales")]
///This icon requires the feature `RiOthersFillScales` to be enabled.
#[component]
pub fn Scales(
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
        "M13 2v1h7v2h-7v14h4v2H7v-2h4V5H4V3h7V2h2zM5 6.343l2.828 2.829C8.552 9.895 9 10.895 9 12c0 2.21-1.79 4-4 4s-4-1.79-4-4c0-1.105.448-2.105 1.172-2.828L5 6.343zm14 0l2.828 2.829C22.552 9.895 23 10.895 23 12c0 2.21-1.79 4-4 4s-4-1.79-4-4c0-1.105.448-2.105 1.172-2.828L19 6.343zm0 2.829l-1.414 1.414C17.212 10.96 17 11.46 17 12l4 .001c0-.54-.212-1.041-.586-1.415L19 9.172zm-14 0l-1.414 1.414C3.212 10.96 3 11.46 3 12l4 .001c0-.54-.212-1.041-.586-1.415L5 9.172z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
