#[cfg(feature = "SiEquinixmetal")]
use leptos::*;
#[cfg(feature = "SiEquinixmetal")]
///This icon requires the feature `SiEquinixmetal` to be enabled.
#[component]
pub fn Equinixmetal(
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
        "M22.399 7.789v7.005l-1.599.56V7.231L16 5.557v11.472l-1.601.557V4.996L12 4.16l-2.4.836v12.59l-1.599-.557V5.557L3.2 7.232v8.121l-1.599-.56V7.79L0 8.349v7.582l4.801 1.676v-9.24l1.6-.558v10.356L11.2 19.84V6.133l.8-.28.8.28v13.708l4.801-1.676V7.809l1.599.558v9.24L24 15.93V8.349z"
        /> < title > { title } < / title > < / svg >
    }
}
