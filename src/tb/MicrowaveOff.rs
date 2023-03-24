#[cfg(feature = "TbMicrowaveOff")]
use leptos::*;
#[cfg(feature = "TbMicrowaveOff")]
///This icon requires the feature `TbMicrowaveOff` to be enabled.
#[component]
pub fn MicrowaveOff(
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
        "icon icon-tabler icon-tabler-microwave-off" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M18 18h-14a1 1 0 0 1 -1 -1v-10a1 1 0 0 1 1 -1h2m4 0h10a1 1 0 0 1 1 1v10" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M15 6v5m0 4v3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18 12h.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18 9h.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M6.5 10.5c1 -.667 1.5 -.667 2.5 0c.636 .265 1.272 .665 1.907 .428" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M6.5 13.5c1 -.667 1.5 -.667 2.5 0c.833 .347 1.667 .926 2.5 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
