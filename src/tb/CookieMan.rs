#[cfg(feature = "TbCookieMan")]
use leptos::*;
#[cfg(feature = "TbCookieMan")]
///This icon requires the feature `TbCookieMan` to be enabled.
#[component]
pub fn CookieMan(
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
        "icon icon-tabler icon-tabler-cookie-man" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 2a5 5 0 0 1 2.845 9.112l.147 .369l1.755 -.803c.969 -.443 2.12 -.032 2.571 .918a1.88 1.88 0 0 1 -.787 2.447l-.148 .076l-2.383 1.089v2.02l1.426 1.425l.114 .125a1.96 1.96 0 0 1 -2.762 2.762l-.125 -.114l-2.079 -2.08l-.114 -.124a1.957 1.957 0 0 1 -.161 -.22h-.599c-.047 .075 -.101 .15 -.16 .22l-.115 .125l-2.08 2.079a1.96 1.96 0 0 1 -2.886 -2.648l.114 -.125l1.427 -1.426v-2.019l-2.383 -1.09l-.148 -.075a1.88 1.88 0 0 1 -.787 -2.447c.429 -.902 1.489 -1.318 2.424 -.978l.147 .06l1.755 .803l.147 -.369a5 5 0 0 1 -2.15 -3.895l0 -.217a5 5 0 0 1 5 -5z"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M12 16h.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 13h.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10 7h.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14 7h.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 9h.01" /> < title > { title } < / title > <
        / svg >
    }
}
