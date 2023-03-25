#[cfg(feature = "SiLevelsdotfyi")]
use leptos::*;
#[cfg(feature = "SiLevelsdotfyi")]
///This icon requires the feature `SiLevelsdotfyi` to be enabled.
#[component]
pub fn Levelsdotfyi(
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
        "M2.494 18.913h3.52v-3.52c0-.43.35-.78.781-.78h3.52v-3.52c0-.432.35-.783.781-.783h3.52V6.791c0-.432.35-.782.782-.782h3.519V2.49c0-.432.35-.782.782-.782h3.52c.43 0 .781.35.781.782v20.724c0 .432-.35.782-.782.782H2.494a.782.782 0 0 1-.782-.782v-3.52c0-.43.35-.78.782-.78ZM.172 15.928a.587.587 0 0 1 0-.83L15.102.168a.587.587 0 0 1 .83.83l-14.93 14.93c-.23.23-.6.23-.83 0Z"
        /> < title > { title } < / title > < / svg >
    }
}
