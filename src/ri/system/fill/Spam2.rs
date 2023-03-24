#[cfg(feature = "RiSystemFillSpam2")]
use leptos::*;
#[cfg(feature = "RiSystemFillSpam2")]
///This icon requires the feature `RiSystemFillSpam2` to be enabled.
#[component]
pub fn Spam2(
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
        "M16.218 2.5l5.683 5.682v8.036l-5.683 5.683H8.182l-5.683-5.683V8.182l5.683-5.683h8.036zM11 15v2h2v-2h-2zm0-8v6h2V7h-2z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
