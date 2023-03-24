#[cfg(feature = "SiCoda")]
use leptos::*;
#[cfg(feature = "SiCoda")]
///This icon requires the feature `SiCoda` to be enabled.
#[component]
pub fn Coda(
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
        "M21.194 0H2.806A2.01 2.01 0 0 0 .8 2v20c0 1.1.903 2 2.006 2h18.388a2.01 2.01 0 0 0 2.006-2v-.933c-.033-1.2-.067-3.7-.067-4.834 0-.633-.468-1.166-1.07-1.166-.668 0-1.103.4-1.437.733-1.003.9-2.508 1.067-3.812.833-.601-.133-1.17-.3-1.638-.6-1.438-.833-2.374-2.4-2.374-4.066 0-1.667.936-3.2 2.374-4.067.502-.3 1.07-.467 1.638-.6 1.27-.233 2.809-.067 3.812.833.367.334.802.734 1.437.734.602 0 1.07-.534 1.07-1.167 0-1.1.034-3.633.067-4.833V2c0-1.1-.903-2-2.006-2Z"
        /> < title > { title } < / title > < / svg >
    }
}
