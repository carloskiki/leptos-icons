#[cfg(feature = "TbLanguage")]
use leptos::*;
#[cfg(feature = "TbLanguage")]
///This icon requires the feature `TbLanguage` to be enabled.
#[component]
pub fn Language(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-language"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M4 5h7" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 3v2c0 4.418 -2.239 8 -5 8" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M5 9c0 2.144 2.952 3.908 6.7 4" />< path xmlns
        = "http://www.w3.org/2000/svg" d = "M12 20l4 -9l4 9" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19.1 18h-6.2" /> < title > { title } < / title
        > < / svg >
    }
}
