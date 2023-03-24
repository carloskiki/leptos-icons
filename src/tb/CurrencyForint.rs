#[cfg(feature = "TbCurrencyForint")]
use leptos::*;
#[cfg(feature = "TbCurrencyForint")]
///This icon requires the feature `TbCurrencyForint` to be enabled.
#[component]
pub fn CurrencyForint(
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
        "icon icon-tabler icon-tabler-currency-forint" width = "24" height = "24" viewBox
        = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M11 4h-4a3 3 0 0 0 -3 3v12"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M10 11h-6" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 4v13a2 2 0 0 0 2 2h2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19 9h-5" /> < title > { title } < / title > <
        / svg >
    }
}
