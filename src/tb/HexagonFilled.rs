#[cfg(feature = "TbHexagonFilled")]
use leptos::*;
#[cfg(feature = "TbHexagonFilled")]
///This icon requires the feature `TbHexagonFilled` to be enabled.
#[component]
pub fn HexagonFilled(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-hexagon-filled" width = "24" height = "24" viewBox
        = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10.543 2.426l-6.026 3.587a2.987 2.987 0 0 0 -1.517 2.607v6.537a3 3 0 0 0 1.544 2.621l5.947 3.802c.958 .534 2.06 .534 2.966 .031l6.052 -3.864c.9 -.498 1.49 -1.501 1.491 -2.59v-6.537l-.005 -.2a2.998 2.998 0 0 0 -1.162 -2.19l-.113 -.083a1.073 1.073 0 0 0 -.18 -.133l-6.025 -3.588a3.056 3.056 0 0 0 -2.972 0z"
        stroke - width = "0" fill = "currentColor" /> < title > { title } < / title > < /
        svg >
    }
}
