#[cfg(feature = "TbTelescope")]
use leptos::*;
#[cfg(feature = "TbTelescope")]
///This icon requires the feature `TbTelescope` to be enabled.
#[component]
pub fn Telescope(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-telescope"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d = "M6 21l6 -5l6 5"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M12 13v8" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M3.294 13.678l.166 .281c.52 .88 1.624 1.265 2.605 .91l14.242 -5.165a1.023 1.023 0 0 0 .565 -1.456l-2.62 -4.705a1.087 1.087 0 0 0 -1.447 -.42l-.056 .032l-12.694 7.618c-1.02 .613 -1.357 1.897 -.76 2.905z"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M14 5l3 5.5" /> < title > {
        title } < / title > < / svg >
    }
}
