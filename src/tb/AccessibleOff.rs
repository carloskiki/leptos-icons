#[cfg(feature = "TbAccessibleOff")]
use leptos::*;
#[cfg(feature = "TbAccessibleOff")]
///This icon requires the feature `TbAccessibleOff` to be enabled.
#[component]
pub fn AccessibleOff(
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
        "icon icon-tabler icon-tabler-accessible-off" width = "24" height = "24" viewBox
        = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10 16.5l2 -3l2 3m-2 -3v-1.5m2.627 -1.376l.373 -.124m-6 0l2.231 .744" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M20.042 16.045a9 9 0 0 0 -12.087 -12.087m-2.318 1.677a9 9 0 1 0 12.725 12.73"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M12 8a.5 .5 0 1 0 -.5 -.5" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title }
        < / title > < / svg >
    }
}
