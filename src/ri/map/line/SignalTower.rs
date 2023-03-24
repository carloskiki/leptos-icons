#[cfg(feature = "RiMapLineSignalTower")]
use leptos::*;
#[cfg(feature = "RiMapLineSignalTower")]
///This icon requires the feature `RiMapLineSignalTower` to be enabled.
#[component]
pub fn SignalTower(
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
        "M6.116 20.087A9.986 9.986 0 0 1 2 12C2 6.477 6.477 2 12 2s10 4.477 10 10a9.986 9.986 0 0 1-4.116 8.087l-1.015-1.739a8 8 0 1 0-9.738 0l-1.015 1.739zm2.034-3.485a6 6 0 1 1 7.7 0l-1.03-1.766a4 4 0 1 0-5.64 0l-1.03 1.766zM11 13h2v9h-2v-9z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
