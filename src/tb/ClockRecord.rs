#[cfg(feature = "TbClockRecord")]
use leptos::*;
#[cfg(feature = "TbClockRecord")]
///This icon requires the feature `TbClockRecord` to be enabled.
#[component]
pub fn ClockRecord(
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
        "icon icon-tabler icon-tabler-clock-record" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M21 12.3a9 9 0 1 0 -8.683 8.694" />< path xmlns = "http://www.w3.org/2000/svg" d
        = "M12 7v5l2 2" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M19 19m-3 0a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> < title > { title } < / title > < /
        svg >
    }
}
