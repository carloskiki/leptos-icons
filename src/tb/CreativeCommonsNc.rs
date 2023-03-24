#[cfg(feature = "TbCreativeCommonsNc")]
use leptos::*;
#[cfg(feature = "TbCreativeCommonsNc")]
///This icon requires the feature `TbCreativeCommonsNc` to be enabled.
#[component]
pub fn CreativeCommonsNc(
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
        "icon icon-tabler icon-tabler-creative-commons-nc" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d
        = "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 12m-9 0a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M15 9h-4.5a1.5 1.5 0 0 0 0 3h3a1.5 1.5 0 0 1 0 3h-4.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 7v2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 15v2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M6 6l3 3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 15l3 3" /> < title > { title } < / title >
        < / svg >
    }
}
