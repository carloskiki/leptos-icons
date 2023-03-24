#[cfg(feature = "SiFig")]
use leptos::*;
#[cfg(feature = "SiFig")]
///This icon requires the feature `SiFig` to be enabled.
#[component]
pub fn Fig(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M4.543 0A4.543 4.543 0 0 0 0 4.543v14.914A4.543 4.543 0 0 0 4.543 24h14.914A4.543 4.543 0 0 0 24 19.457V4.543A4.543 4.543 0 0 0 19.457 0H4.543zm10.715 2.572h3.086a3.326 3.326 0 0 1 .656.065 3.28 3.28 0 0 1 .898.326c.093.05.181.106.268.164.087.058.172.12.252.186a3.264 3.264 0 0 1 .789.96c.05.093.098.188.139.286a3.21 3.21 0 0 1 .187.61 3.276 3.276 0 0 1 .067.66v12.343a3.276 3.276 0 0 1-.146.97 3.21 3.21 0 0 1-.108.3 3.259 3.259 0 0 1-.301.552 3.28 3.28 0 0 1-.627.694c-.08.066-.165.127-.252.185a3.242 3.242 0 0 1-.553.3 3.222 3.222 0 0 1-1.27.255h-3.085a3.326 3.326 0 0 1-.658-.065 3.242 3.242 0 0 1-2.344-1.922 3.222 3.222 0 0 1-.256-1.27V5.829a3.324 3.324 0 0 1 .393-1.555 3.221 3.221 0 0 1 1.04-1.146 3.241 3.241 0 0 1 .553-.3 3.26 3.26 0 0 1 1.272-.255z"
        /> < title > { title } < / title > < / svg >
    }
}
