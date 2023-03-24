#[cfg(feature = "AiOutlinedDotChart")]
use leptos::*;
#[cfg(feature = "AiOutlinedDotChart")]
///This icon requires the feature `AiOutlinedDotChart` to be enabled.
#[component]
pub fn DotChart(
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
        stroke_witdh = "0" style = style class = "icon" viewBox = "0 0 1024 1024" width =
        size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M888 792H200V168c0-4.4-3.6-8-8-8h-56c-4.4 0-8 3.6-8 8v688c0 4.4 3.6 8 8 8h752c4.4 0 8-3.6 8-8v-56c0-4.4-3.6-8-8-8zM288 604a64 64 0 1 0 128 0 64 64 0 1 0-128 0zm118-224a48 48 0 1 0 96 0 48 48 0 1 0-96 0zm158 228a96 96 0 1 0 192 0 96 96 0 1 0-192 0zm148-314a56 56 0 1 0 112 0 56 56 0 1 0-112 0z"
        /> < title > { title } < / title > < / svg >
    }
}
