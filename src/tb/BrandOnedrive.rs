#[cfg(feature = "TbBrandOnedrive")]
use leptos::*;
#[cfg(feature = "TbBrandOnedrive")]
///This icon requires the feature `TbBrandOnedrive` to be enabled.
#[component]
pub fn BrandOnedrive(
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
        "icon icon-tabler icon-tabler-brand-onedrive" width = "24" height = "24" viewBox
        = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M18.456 10.45a6.45 6.45 0 0 0 -12 -2.151a4.857 4.857 0 0 0 -4.44 5.241a4.856 4.856 0 0 0 5.236 4.444h10.751a3.771 3.771 0 0 0 3.99 -3.54a3.772 3.772 0 0 0 -3.538 -3.992z"
        /> < title > { title } < / title > < / svg >
    }
}
