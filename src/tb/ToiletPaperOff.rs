#[cfg(feature = "TbToiletPaperOff")]
use leptos::*;
#[cfg(feature = "TbToiletPaperOff")]
///This icon requires the feature `TbToiletPaperOff` to be enabled.
#[component]
pub fn ToiletPaperOff(
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
        "icon icon-tabler icon-tabler-toilet-paper-off" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M4.27 4.28c-.768 1.27 -1.27 3.359 -1.27 5.72c0 3.866 1.343 7 3 7s3 -3.134 3 -7c0 -.34 -.01 -.672 -.03 -1"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M21 10c0 -3.866 -1.343 -7 -3 -7" />< path xmlns = "http://www.w3.org/2000/svg" d
        = "M7 3h11" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M21 10v7m-1.513 2.496l-1.487 -.496l-3 2l-3 -3l-3 2v-10" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M6 10h.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
