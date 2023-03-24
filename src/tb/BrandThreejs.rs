#[cfg(feature = "TbBrandThreejs")]
use leptos::*;
#[cfg(feature = "TbBrandThreejs")]
///This icon requires the feature `TbBrandThreejs` to be enabled.
#[component]
pub fn BrandThreejs(
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
        "icon icon-tabler icon-tabler-brand-threejs" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M8 22l-5 -19l19 5.5z" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M12.573 17.58l-6.152 -1.576l8.796 -9.466l1.914 6.64" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12.573 17.58l-1.573 -6.58l6.13 2.179" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M9.527 4.893l1.473 6.107l-6.31 -1.564z"
        /> < title > { title } < / title > < / svg >
    }
}
