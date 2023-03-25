#[cfg(feature = "TbPhotoUp")]
use leptos::*;
#[cfg(feature = "TbPhotoUp")]
///This icon requires the feature `TbPhotoUp` to be enabled.
#[component]
pub fn PhotoUp(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-photo-up"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d = "M15 8h.01" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 20h-5a3 3 0 0 1 -3 -3v-10a3 3 0 0 1 3 -3h10a3 3 0 0 1 3 3v5" />< path xmlns
        = "http://www.w3.org/2000/svg" d = "M4 15l4 -4c.928 -.893 2.072 -.893 3 0l4 4"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M14 14l1 -1c.617 -.593 1.328 -.793 2.009 -.598" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19 22v-6" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M22 19l-3 -3l-3 3" /> < title > { title } < /
        title > < / svg >
    }
}
