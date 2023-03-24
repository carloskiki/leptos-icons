#[cfg(feature = "TbBrandCake")]
use leptos::*;
#[cfg(feature = "TbBrandCake")]
///This icon requires the feature `TbBrandCake` to be enabled.
#[component]
pub fn BrandCake(
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
        "icon icon-tabler icon-tabler-brand-cake" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M20.84 12c0 2.05 .985 3.225 -.04 5c-1.026 1.775 -2.537 1.51 -4.314 2.534c-1.776 1.026 -2.302 2.466 -4.353 2.466c-2.051 0 -2.576 -1.441 -4.353 -2.466c-1.776 -1.024 -3.288 -.759 -4.314 -2.534c-1.025 -1.775 -.04 -2.95 -.04 -5s-.985 -3.225 .04 -5c1.026 -1.775 2.537 -1.51 4.314 -2.534c1.776 -1.026 2.302 -2.466 4.353 -2.466s2.577 1.441 4.353 2.466c1.776 1.024 3.288 .759 4.313 2.534c1.026 1.775 .04 2.95 .04 5z"
        /> < title > { title } < / title > < / svg >
    }
}
