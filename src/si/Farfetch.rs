#[cfg(feature = "SiFarfetch")]
use leptos::*;
#[cfg(feature = "SiFarfetch")]
///This icon requires the feature `SiFarfetch` to be enabled.
#[component]
pub fn Farfetch(
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
        "M24 10.248V6.749H13.586c-3.062 0-4.737 1.837-4.737 4.488v2.231H4.321V8.599c0-3.425.332-5.074 4.212-5.074H24V0H6.259C2.336 0 0 2.589 0 6.386V24h4.321v-7.033h4.527V24h4.339v-7.033H24v-3.499H13.188v-1.155c0-1.461.232-2.064 2.257-2.064H24z"
        /> < title > { title } < / title > < / svg >
    }
}
