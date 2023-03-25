#[cfg(feature = "SiKaios")]
use leptos::*;
#[cfg(feature = "SiKaios")]
///This icon requires the feature `SiKaios` to be enabled.
#[component]
pub fn Kaios(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M17.3419.0001a3.6735 3.6735 0 0 0-3.647 3.6735 3.6727 3.6727 0 0 0 3.6735 3.6734 3.6735 3.6735 0 1 0-.0265-7.3469zM4.6233.16a2.7459 2.7459 0 0 0-2.7475 2.7473v18.167a2.7474 2.7474 0 1 0 5.4942 0V2.9071A2.749 2.749 0 0 0 4.6233.16zm6.9494 7.2078a2.729 2.729 0 0 0-2.237 4.2947l7.8107 11.1541a2.729 2.729 0 1 0 4.4706-3.1307L13.8062 8.5311a2.729 2.729 0 0 0-2.2335-1.1634z"
        /> < title > { title } < / title > < / svg >
    }
}
