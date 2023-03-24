#[cfg(feature = "TbRollercoaster")]
use leptos::*;
#[cfg(feature = "TbRollercoaster")]
///This icon requires the feature `TbRollercoaster` to be enabled.
#[component]
pub fn Rollercoaster(
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
        "icon icon-tabler icon-tabler-rollercoaster" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3 21a5.55 5.55 0 0 0 5.265 -3.795l.735 -2.205a8.775 8.775 0 0 1 8.325 -6h3.675"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M20 9v12" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8 21v-3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 21v-10" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 9.5v11.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 3h5v3h-5z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M6 8l4 -3l2 2.5l-4 3l-1.8 -.5z" /> < title > {
        title } < / title > < / svg >
    }
}
