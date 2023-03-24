#[cfg(feature = "TbAdCircleOff")]
use leptos::*;
#[cfg(feature = "TbAdCircleOff")]
///This icon requires the feature `TbAdCircleOff` to be enabled.
#[component]
pub fn AdCircleOff(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-ad-circle-off" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M4.91 4.949a9.968 9.968 0 0 0 -2.91 7.051c0 5.523 4.477 10 10 10a9.968 9.968 0 0 0 7.05 -2.909"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M20.778 16.793a9.955 9.955 0 0 0 1.222 -4.793c0 -5.523 -4.477 -10 -10 -10c-1.74 0 -3.376 .444 -4.8 1.225"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M7 15v-4.5a1.5 1.5 0 0 1 2.138 -1.358" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9.854 9.853c.094 .196 .146 .415 .146 .647v4.5"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M7 13h3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14 14v1h1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17 13v-2a2 2 0 0 0 -2 -2h-1v1" />< path xmlns
        = "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title
        > < / svg >
    }
}
