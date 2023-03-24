#[cfg(feature = "TbBrandBunpo")]
use leptos::*;
#[cfg(feature = "TbBrandBunpo")]
///This icon requires the feature `TbBrandBunpo` to be enabled.
#[component]
pub fn BrandBunpo(
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
        "icon icon-tabler icon-tabler-brand-bunpo" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3.9 7.205a17.764 17.764 0 0 0 4.008 2.753a7.917 7.917 0 0 0 4.57 .567c1.5 -.33 2.907 -1 4.121 -1.956a12.107 12.107 0 0 0 2.892 -2.903c.603 -.94 .745 -1.766 .484 -2.231c-.261 -.465 -.927 -.568 -1.72 -.257a7.564 7.564 0 0 0 -2.608 2.034a18.425 18.425 0 0 0 -2.588 3.884a34.927 34.927 0 0 0 -2.093 5.073a12.908 12.908 0 0 0 -.677 3.515c-.07 .752 .07 1.51 .405 2.184c.323 .562 1.06 1.132 2.343 1.132c3.474 0 5.093 -3.53 5.463 -5.62c.24 -1.365 -.085 -3.197 -1.182 -4.01"
        /> < title > { title } < / title > < / svg >
    }
}
