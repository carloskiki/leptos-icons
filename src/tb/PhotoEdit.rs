#[cfg(feature = "TbPhotoEdit")]
use leptos::*;
#[cfg(feature = "TbPhotoEdit")]
///This icon requires the feature `TbPhotoEdit` to be enabled.
#[component]
pub fn PhotoEdit(
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
        "icon icon-tabler icon-tabler-photo-edit" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M15 8h.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M11 20h-4a3 3 0 0 1 -3 -3v-10a3 3 0 0 1 3 -3h10a3 3 0 0 1 3 3v4" />< path xmlns
        = "http://www.w3.org/2000/svg" d = "M4 15l4 -4c.928 -.893 2.072 -.893 3 0l3 3"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M14 14l1 -1c.31 -.298 .644 -.497 .987 -.596" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M18.42 15.61a2.1 2.1 0 0 1 2.97 2.97l-3.39 3.42h-3v-3l3.42 -3.39z" /> < title >
        { title } < / title > < / svg >
    }
}
