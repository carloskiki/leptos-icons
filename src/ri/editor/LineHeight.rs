#[cfg(feature = "RiEditorLineHeight")]
use leptos::*;
#[cfg(feature = "RiEditorLineHeight")]
///This icon requires the feature `RiEditorLineHeight` to be enabled.
#[component]
pub fn LineHeight(
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
        "M11 4h10v2H11V4zM6 7v4H4V7H1l4-4 4 4H6zm0 10h3l-4 4-4-4h3v-4h2v4zm5 1h10v2H11v-2zm-2-7h12v2H9v-2z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
