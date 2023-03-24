#[cfg(feature = "TbAd2")]
use leptos::*;
#[cfg(feature = "TbAd2")]
///This icon requires the feature `TbAd2` to be enabled.
#[component]
pub fn Ad2(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-ad-2"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M11.933 5h-6.933v16h13v-8" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M14 17h-5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 13h5v-4h-5z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 5v-2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18 6l2 -2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19 9h2" /> < title > { title } < / title > < /
        svg >
    }
}
