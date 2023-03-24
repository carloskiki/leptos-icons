#[cfg(feature = "TbRulerMeasure")]
use leptos::*;
#[cfg(feature = "TbRulerMeasure")]
///This icon requires the feature `TbRulerMeasure` to be enabled.
#[component]
pub fn RulerMeasure(
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
        "icon icon-tabler icon-tabler-ruler-measure" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M19.875 12c.621 0 1.125 .512 1.125 1.143v5.714c0 .631 -.504 1.143 -1.125 1.143h-15.875a1 1 0 0 1 -1 -1v-5.857c0 -.631 .504 -1.143 1.125 -1.143h15.75z"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M9 12v2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M6 12v3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 12v3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18 12v3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 12v2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3v4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 5h18" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M21 3v4" /> < title > { title } < / title > < /
        svg >
    }
}
