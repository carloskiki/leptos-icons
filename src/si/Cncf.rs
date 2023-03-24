#[cfg(feature = "SiCncf")]
use leptos::*;
#[cfg(feature = "SiCncf")]
///This icon requires the feature `SiCncf` to be enabled.
#[component]
pub fn Cncf(
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
        "M0 0v24h24V0H8.004Zm3.431 3.431h4.544l.029.023 4.544 4.544h3.03l-4.572-4.567h9.569v9.563l-.789-.782-3.784-3.79v3.03l2.271 2.272 2.272 2.272.029.03v4.543h-4.55l-.023-.023-2.272-2.278-2.272-2.272H8.427l3.785 3.79.782.783H3.43v-9.563l4.573 4.567v-3.031l-4.55-4.544-.023-.023Z"
        /> < title > { title } < / title > < / svg >
    }
}
