#[cfg(feature = "SiTasmota")]
use leptos::*;
#[cfg(feature = "SiTasmota")]
///This icon requires the feature `SiTasmota` to be enabled.
#[component]
pub fn Tasmota(
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
        "M12 0L0 12l1.371 1.372L12 2.743l10.629 10.629L24 12 12 0zm0 8.463a7.41 7.41 0 0 0-2.64 14.334v-2.133a5.464 5.464 0 0 1 1.671-10.17V24h1.94V10.494a5.464 5.464 0 0 1 1.669 10.171v2.133A7.41 7.41 0 0 0 12 8.463z"
        /> < title > { title } < / title > < / svg >
    }
}
