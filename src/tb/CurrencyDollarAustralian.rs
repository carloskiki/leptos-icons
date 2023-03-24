#[cfg(feature = "TbCurrencyDollarAustralian")]
use leptos::*;
#[cfg(feature = "TbCurrencyDollarAustralian")]
///This icon requires the feature `TbCurrencyDollarAustralian` to be enabled.
#[component]
pub fn CurrencyDollarAustralian(
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
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-currency-dollar-australian" width = "24" height =
        "24" viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill =
        "none" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3 18l3.279 -11.476a.75 .75 0 0 1 1.442 0l3.279 11.476" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M21 6h-4a3 3 0 0 0 0 6h1a3 3 0 0 1 0 6h-4" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M17 20v-2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18 6v-2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4.5 14h5" /> < title > { title } < / title > <
        / svg >
    }
}
