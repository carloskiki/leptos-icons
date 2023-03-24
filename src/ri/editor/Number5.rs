#[cfg(feature = "RiEditorNumber5")]
use leptos::*;
#[cfg(feature = "RiEditorNumber5")]
///This icon requires the feature `RiEditorNumber5` to be enabled.
#[component]
pub fn Number5(
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
        "M18 2v2H9.3l-.677 6.445a6.5 6.5 0 1 1-2.93 7.133l1.94-.486A4.502 4.502 0 0 0 16.5 16a4.5 4.5 0 0 0-4.5-4.5c-2.022 0-3.278.639-3.96 1.53l-1.575-1.182L7.5 2H18z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
