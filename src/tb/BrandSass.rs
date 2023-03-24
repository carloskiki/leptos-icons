#[cfg(feature = "TbBrandSass")]
use leptos::*;
#[cfg(feature = "TbBrandSass")]
///This icon requires the feature `TbBrandSass` to be enabled.
#[component]
pub fn BrandSass(
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
        "icon icon-tabler icon-tabler-brand-sass" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 12m-9 0a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M12 10.523c2.46 -.826 4 -.826 4 -2.155c0 -1.366 -1.347 -1.366 -2.735 -1.366c-1.91 0 -3.352 .49 -4.537 1.748c-.848 .902 -1.027 2.449 -.153 3.307c.973 .956 3.206 1.789 2.884 3.493c-.233 1.235 -1.469 1.823 -2.617 1.202c-.782 -.424 -.454 -1.746 .626 -2.512s2.822 -.992 4.1 -.24c.98 .575 1.046 1.724 .434 2.193"
        /> < title > { title } < / title > < / svg >
    }
}
