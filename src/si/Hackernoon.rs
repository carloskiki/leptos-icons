#[cfg(feature = "SiHackernoon")]
use leptos::*;
#[cfg(feature = "SiHackernoon")]
///This icon requires the feature `SiHackernoon` to be enabled.
#[component]
pub fn Hackernoon(
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
        "M5.701 0v6.223H8.85V4.654h1.576v7.842H12V4.654h1.574v1.569h3.15V0zm11.024 6.223v3.136h1.574V6.223zm1.574 3.136v4.705h1.576v-1.568h1.574v-1.568h-1.574V9.359zm0 4.705h-1.574v3.137h1.574zm-1.574 3.137h-3.15v1.569H8.85V17.2H5.7V24h11.024zm-11.024 0v-3.137H4.125v3.137zm-1.576-3.137V9.36H2.551v4.705zm0-4.705h1.576V6.223H4.125Z"
        /> < title > { title } < / title > < / svg >
    }
}
