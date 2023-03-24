#[cfg(feature = "RiOthersLineRecycle")]
use leptos::*;
#[cfg(feature = "RiOthersLineRecycle")]
///This icon requires the feature `RiOthersLineRecycle` to be enabled.
#[component]
pub fn Recycle(
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
        "M19.562 12.097l1.531 2.653c.967 1.674.393 3.815-1.28 4.781-.533.307-1.136.469-1.75.469H16v2.5L11 19l5-3.5V18h2.062c.263 0 .522-.07.75-.201.718-.414.963-1.332.55-2.049l-1.532-2.653 1.732-1zM7.304 9.134l.53 6.08-2.164-1.25-1.031 1.786c-.132.228-.201.487-.201.75 0 .828.671 1.5 1.5 1.5H9v2H5.938c-1.933 0-3.5-1.567-3.5-3.5 0-.614.162-1.218.469-1.75l1.03-1.787-2.164-1.249 5.53-2.58zm6.446-6.165c.532.307.974.749 1.281 1.281l1.03 1.785 2.166-1.25-.53 6.081-5.532-2.58 2.165-1.25-1.031-1.786c-.132-.228-.321-.417-.549-.549-.717-.414-1.635-.168-2.049.549L9.169 7.903l-1.732-1L8.97 4.25c.966-1.674 3.107-2.248 4.781-1.281z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
