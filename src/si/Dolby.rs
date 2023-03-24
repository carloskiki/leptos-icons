#[cfg(feature = "SiDolby")]
use leptos::*;
#[cfg(feature = "SiDolby")]
///This icon requires the feature `SiDolby` to be enabled.
#[component]
pub fn Dolby(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M24,20.352V3.648H0v16.704H24z M18.433,5.806h2.736v12.387h-2.736c-2.839,0-5.214-2.767-5.214-6.194S15.594,5.806,18.433,5.806z M2.831,5.806h2.736c2.839,0,5.214,2.767,5.214,6.194s-2.374,6.194-5.214,6.194H2.831V5.806z"
        /> < title > { title } < / title > < / svg >
    }
}
