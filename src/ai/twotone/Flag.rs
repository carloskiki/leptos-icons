#[cfg(feature = "AiTwotoneFlag")]
use leptos::*;
#[cfg(feature = "AiTwotoneFlag")]
///This icon requires the feature `AiTwotoneFlag` to be enabled.
#[component]
pub fn Flag(
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
        stroke_witdh = "0" style = style viewBox = "0 0 1024 1024" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" fill = "#D9D9D9" d = "M184 232h368v336H184z" /><
        path xmlns = "http://www.w3.org/2000/svg" fill = "#D9D9D9" d =
        "M624 632c0 4.4-3.6 8-8 8H504v73h336V377H624v255z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M880 305H624V192c0-17.7-14.3-32-32-32H184v-40c0-4.4-3.6-8-8-8h-56c-4.4 0-8 3.6-8 8v784c0 4.4 3.6 8 8 8h56c4.4 0 8-3.6 8-8V640h248v113c0 17.7 14.3 32 32 32h416c17.7 0 32-14.3 32-32V337c0-17.7-14.3-32-32-32zM184 568V232h368v336H184zm656 145H504v-73h112c4.4 0 8-3.6 8-8V377h216v336z"
        /> < title > { title } < / title > < / svg >
    }
}
