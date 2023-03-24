#[cfg(feature = "TbTestPipeOff")]
use leptos::*;
#[cfg(feature = "TbTestPipeOff")]
///This icon requires the feature `TbTestPipeOff` to be enabled.
#[component]
pub fn TestPipeOff(
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
        "icon icon-tabler icon-tabler-test-pipe-off" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M20 8.04a803.533 803.533 0 0 0 -4 3.96m-2 2c-1.085 1.085 -3.125 3.14 -6.122 6.164a2.857 2.857 0 0 1 -4.041 -4.04c3.018 -3 5.073 -5.037 6.163 -6.124m2 -2c.872 -.872 2.191 -2.205 3.959 -4"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M7 13h6" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M19 15l1.5 1.6m-.74 3.173a2 2 0 0 1 -2.612 -2.608" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 3l6 6" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
