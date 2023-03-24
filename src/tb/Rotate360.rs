#[cfg(feature = "TbRotate360")]
use leptos::*;
#[cfg(feature = "TbRotate360")]
///This icon requires the feature `TbRotate360` to be enabled.
#[component]
pub fn Rotate360(
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
        "icon icon-tabler icon-tabler-rotate-360" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M12 16h4v4" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M19.458 11.042c.86 -2.366 .722 -4.58 -.6 -5.9c-2.272 -2.274 -7.185 -1.045 -10.973 2.743c-3.788 3.788 -5.017 8.701 -2.744 10.974c2.227 2.226 6.987 1.093 10.74 -2.515"
        /> < title > { title } < / title > < / svg >
    }
}
