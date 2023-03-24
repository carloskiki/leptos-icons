#[cfg(feature = "IoCellular")]
use leptos::*;
#[cfg(feature = "IoCellular")]
///This icon requires the feature `IoCellular` to be enabled.
#[component]
pub fn Cellular(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M472,432H424a24,24,0,0,1-24-24V104a24,24,0,0,1,24-24h48a24,24,0,0,1,24,24V408A24,24,0,0,1,472,432Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M344,432H296a24,24,0,0,1-24-24V184a24,24,0,0,1,24-24h48a24,24,0,0,1,24,24V408A24,24,0,0,1,344,432Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M216,432H168a24,24,0,0,1-24-24V248a24,24,0,0,1,24-24h48a24,24,0,0,1,24,24V408A24,24,0,0,1,216,432Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M88,432H40a24,24,0,0,1-24-24V312a24,24,0,0,1,24-24H88a24,24,0,0,1,24,24v96A24,24,0,0,1,88,432Z"
        /> < title > { title } < / title > < / svg >
    }
}
