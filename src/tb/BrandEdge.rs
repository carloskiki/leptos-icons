#[cfg(feature = "TbBrandEdge")]
use leptos::*;
#[cfg(feature = "TbBrandEdge")]
///This icon requires the feature `TbBrandEdge` to be enabled.
#[component]
pub fn BrandEdge(
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
        "icon icon-tabler icon-tabler-brand-edge" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M20.978 11.372a9 9 0 1 0 -1.593 5.773" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M20.978 11.372c.21 2.993 -5.034 2.413 -6.913 1.486c1.392 -1.6 .402 -4.038 -2.274 -3.851c-1.745 .122 -2.927 1.157 -2.784 3.202c.28 3.99 4.444 6.205 10.36 4.79"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3.022 12.628c-.283 -4.043 8.717 -7.228 11.248 -2.688" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M12.628 20.978c-2.993 .21 -5.162 -4.725 -3.567 -9.748" /> < title > { title } <
        / title > < / svg >
    }
}
