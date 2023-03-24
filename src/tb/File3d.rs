#[cfg(feature = "TbFile3d")]
use leptos::*;
#[cfg(feature = "TbFile3d")]
///This icon requires the feature `TbFile3d` to be enabled.
#[component]
pub fn File3d(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-file-3d"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M14 3v4a1 1 0 0 0 1 1h4" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2z" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M12 13.5l4 -1.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M8 11.846l4 1.654v4.5l4 -1.846v-4.308l-4 -1.846z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8 12v4.2l4 1.8" /> < title > { title } < /
        title > < / svg >
    }
}
