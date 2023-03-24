#[cfg(feature = "TbMoustache")]
use leptos::*;
#[cfg(feature = "TbMoustache")]
///This icon requires the feature `TbMoustache` to be enabled.
#[component]
pub fn Moustache(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-moustache"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15 9a3 3 0 0 1 2.599 1.5h0c.933 1.333 2.133 1.556 3.126 1.556l.291 0l.77 -.044l.213 0c-.963 1.926 -3.163 2.925 -6.6 3l-.4 0l-.165 0a3 3 0 0 1 .165 -6z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M9 9a3 3 0 0 0 -2.599 1.5h0c-.933 1.333 -2.133 1.556 -3.126 1.556l-.291 0l-.77 -.044l-.213 0c.963 1.926 3.163 2.925 6.6 3l.4 0l.165 0a3 3 0 0 0 -.165 -6z"
        /> < title > { title } < / title > < / svg >
    }
}
