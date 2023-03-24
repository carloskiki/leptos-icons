#[cfg(feature = "TbBucket")]
use leptos::*;
#[cfg(feature = "TbBucket")]
///This icon requires the feature `TbBucket` to be enabled.
#[component]
pub fn Bucket(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-bucket"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 7m-8 0a8 4 0 1 0 16 0a8 4 0 1 0 -16 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M4 7c0 .664 .088 1.324 .263 1.965l2.737 10.035c.5 1.5 2.239 2 5 2s4.5 -.5 5 -2c.333 -1 1.246 -4.345 2.737 -10.035a7.45 7.45 0 0 0 .263 -1.965"
        /> < title > { title } < / title > < / svg >
    }
}
