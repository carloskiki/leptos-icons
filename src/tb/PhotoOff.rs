#[cfg(feature = "TbPhotoOff")]
use leptos::*;
#[cfg(feature = "TbPhotoOff")]
///This icon requires the feature `TbPhotoOff` to be enabled.
#[component]
pub fn PhotoOff(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-photo-off"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d = "M3 3l18 18" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M15 8l.01 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M19.121 19.122a3 3 0 0 1 -2.121 .878h-10a3 3 0 0 1 -3 -3v-10c0 -.833 .34 -1.587 .888 -2.131m3.112 -.869h9a3 3 0 0 1 3 3v9"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M4 15l4 -4c.928 -.893 2.072 -.893 3 0l5 5" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M16.32 12.34c.577 -.059 1.162 .162 1.68 .66l2 2" /> < title > { title } < /
        title > < / svg >
    }
}
