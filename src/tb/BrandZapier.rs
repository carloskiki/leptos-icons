#[cfg(feature = "TbBrandZapier")]
use leptos::*;
#[cfg(feature = "TbBrandZapier")]
///This icon requires the feature `TbBrandZapier` to be enabled.
#[component]
pub fn BrandZapier(
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
        "icon icon-tabler icon-tabler-brand-zapier" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M3 12h6" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M21 12h-6" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 3v6" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 15v6" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M5.636 5.636l4.243 4.243" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18.364 18.364l-4.243 -4.243" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18.364 5.636l-4.243 4.243" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9.879 14.121l-4.243 4.243" /> < title > {
        title } < / title > < / svg >
    }
}
