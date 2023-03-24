#[cfg(feature = "TbBrandDocker")]
use leptos::*;
#[cfg(feature = "TbBrandDocker")]
///This icon requires the feature `TbBrandDocker` to be enabled.
#[component]
pub fn BrandDocker(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-brand-docker" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M22 12.54c-1.804 -.345 -2.701 -1.08 -3.523 -2.94c-.487 .696 -1.102 1.568 -.92 2.4c.028 .238 -.32 1 -.557 1h-14c0 5.208 3.164 7 6.196 7c4.124 .022 7.828 -1.376 9.854 -5c1.146 -.101 2.296 -1.505 2.95 -2.46z"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M5 10h3v3h-3z" />< path xmlns
        = "http://www.w3.org/2000/svg" d = "M8 10h3v3h-3z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M11 10h3v3h-3z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8 7h3v3h-3z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M11 7h3v3h-3z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M11 4h3v3h-3z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4.571 18c1.5 0 2.047 -.074 2.958 -.78" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M10 16l0 .01" /> < title > { title
        } < / title > < / svg >
    }
}
