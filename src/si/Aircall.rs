#[cfg(feature = "SiAircall")]
use leptos::*;
#[cfg(feature = "SiAircall")]
///This icon requires the feature `SiAircall` to be enabled.
#[component]
pub fn Aircall(
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
        "M23.451 5.906a6.978 6.978 0 0 0-5.375-5.39C16.727.204 14.508 0 12 0S7.273.204 5.924.516a6.978 6.978 0 0 0-5.375 5.39C.237 7.26.034 9.485.034 12s.203 4.74.515 6.094a6.978 6.978 0 0 0 5.375 5.39C7.273 23.796 9.492 24 12 24s4.727-.204 6.076-.516a6.978 6.978 0 0 0 5.375-5.39c.311-1.354.515-3.578.515-6.094 0-2.515-.203-4.74-.515-6.094zm-5.873 12.396l-.003.001c-.428.152-1.165.283-2.102.377l-.147.014a.444.444 0 0 1-.45-.271 1.816 1.816 0 0 0-1.296-1.074c-.351-.081-.928-.134-1.58-.134s-1.229.053-1.58.134a1.817 1.817 0 0 0-1.291 1.062.466.466 0 0 1-.471.281 8 8 0 0 0-.129-.012c-.938-.094-1.676-.224-2.105-.377l-.003-.001a.76.76 0 0 1-.492-.713c0-.032.003-.066.005-.098.073-.979.666-3.272 1.552-5.89C8.5 8.609 9.559 6.187 10.037 5.714a1.029 1.029 0 0 1 .404-.26l.004-.002c.314-.106.892-.178 1.554-.178.663 0 1.241.071 1.554.178l.005.002a1.025 1.025 0 0 1 .405.26c.478.472 1.537 2.895 2.549 5.887.886 2.617 1.479 4.91 1.552 5.89.002.032.005.066.005.098a.76.76 0 0 1-.491.713z"
        /> < title > { title } < / title > < / svg >
    }
}
