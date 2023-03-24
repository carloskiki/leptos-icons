#[cfg(feature = "TbHomeEco")]
use leptos::*;
#[cfg(feature = "TbHomeEco")]
///This icon requires the feature `TbHomeEco` to be enabled.
#[component]
pub fn HomeEco(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-home-eco"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M20 11l-8 -8l-9 9h2v7a2 2 0 0 0 2 2h5" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M9 21v-6a2 2 0 0 1 2 -2h2c.325 0 .631 .077 .902 .215" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 22s0 -2 3 -4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19 21a3 3 0 0 1 0 -6h3v3a3 3 0 0 1 -3 3z" /> <
        title > { title } < / title > < / svg >
    }
}
