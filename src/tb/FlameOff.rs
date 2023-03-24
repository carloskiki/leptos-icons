#[cfg(feature = "TbFlameOff")]
use leptos::*;
#[cfg(feature = "TbFlameOff")]
///This icon requires the feature `TbFlameOff` to be enabled.
#[component]
pub fn FlameOff(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-flame-off"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8.973 8.974c-.335 .378 -.67 .716 -.973 1.026c-1.226 1.26 -2 3.24 -2 5a6 6 0 0 0 11.472 2.466m.383 -3.597c-.32 -1.409 -1.122 -3.045 -1.855 -3.869c-.281 .472 -.543 .87 -.79 1.202m-2.358 -2.35c-.068 -2.157 -1.182 -4.184 -1.852 -4.852c0 .968 -.18 1.801 -.465 2.527"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > {
        title } < / title > < / svg >
    }
}
