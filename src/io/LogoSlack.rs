#[cfg(feature = "IoLogoSlack")]
use leptos::*;
#[cfg(feature = "IoLogoSlack")]
///This icon requires the feature `IoLogoSlack` to be enabled.
#[component]
pub fn LogoSlack(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M126.12,315.1A47.06,47.06,0,1,1,79.06,268h47.06Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M149.84,315.1a47.06,47.06,0,0,1,94.12,0V432.94a47.06,47.06,0,1,1-94.12,0Z" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M196.9,126.12A47.06,47.06,0,1,1,244,79.06v47.06Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M196.9,149.84a47.06,47.06,0,0,1,0,94.12H79.06a47.06,47.06,0,0,1,0-94.12Z" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M385.88,196.9A47.06,47.06,0,1,1,432.94,244H385.88Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M362.16,196.9a47.06,47.06,0,0,1-94.12,0V79.06a47.06,47.06,0,1,1,94.12,0Z" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M315.1,385.88A47.06,47.06,0,1,1,268,432.94V385.88Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M315.1,362.16a47.06,47.06,0,0,1,0-94.12H432.94a47.06,47.06,0,1,1,0,94.12Z" /> <
        title > { title } < / title > < / svg >
    }
}
