#[cfg(feature = "FaSolidLiraSign")]
use leptos::*;
#[cfg(feature = "FaSolidLiraSign")]
///This icon requires the feature `FaSolidLiraSign` to be enabled.
#[component]
pub fn LiraSign(
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
        stroke_witdh = "0" style = style viewBox = "0 0 320 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M112 160.4c0-35.5 28.8-64.4 64.4-64.4c6.9 0 13.8 1.1 20.4 3.3l81.2 27.1c16.8 5.6 34.9-3.5 40.5-20.2s-3.5-34.9-20.2-40.5L217 38.6c-13.1-4.4-26.8-6.6-40.6-6.6C105.5 32 48 89.5 48 160.4V192H32c-17.7 0-32 14.3-32 32s14.3 32 32 32H48v32H32c-17.7 0-32 14.3-32 32s14.3 32 32 32H46c-2.2 10.5-6.1 20.6-11.7 29.9L4.6 431.5c-5.9 9.9-6.1 22.2-.4 32.2S20.5 480 32 480H288c17.7 0 32-14.3 32-32s-14.3-32-32-32H88.5l.7-1.1c11.6-19.3 18.9-40.7 21.6-62.9H224c17.7 0 32-14.3 32-32s-14.3-32-32-32H112V256H224c17.7 0 32-14.3 32-32s-14.3-32-32-32H112V160.4z"
        /> < title > { title } < / title > < / svg >
    }
}
