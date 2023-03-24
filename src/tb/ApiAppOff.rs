#[cfg(feature = "TbApiAppOff")]
use leptos::*;
#[cfg(feature = "TbApiAppOff")]
///This icon requires the feature `TbApiAppOff` to be enabled.
#[component]
pub fn ApiAppOff(
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
        "icon icon-tabler icon-tabler-api-app-off" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 15h-6.5a2.5 2.5 0 1 1 0 -5h.5" />< path xmlns = "http://www.w3.org/2000/svg"
        d = "M15 15v3.5a2.5 2.5 0 1 1 -5 0v-.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M13 9h5.5a2.5 2.5 0 1 1 0 5h-.5" />< path xmlns
        = "http://www.w3.org/2000/svg" d =
        "M9 12v-3m.042 -3.957a2.5 2.5 0 0 1 4.958 .457v.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
