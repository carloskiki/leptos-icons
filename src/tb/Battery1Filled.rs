#[cfg(feature = "TbBattery1Filled")]
use leptos::*;
#[cfg(feature = "TbBattery1Filled")]
///This icon requires the feature `TbBattery1Filled` to be enabled.
#[component]
pub fn Battery1Filled(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-battery-1-filled" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M17 6a3 3 0 0 1 2.995 2.824l.005 .176v.086l.052 .019a1.5 1.5 0 0 1 .941 1.25l.007 .145v3a1.5 1.5 0 0 1 -.948 1.395l-.052 .018v.087a3 3 0 0 1 -2.824 2.995l-.176 .005h-11a3 3 0 0 1 -2.995 -2.824l-.005 -.176v-6a3 3 0 0 1 2.824 -2.995l.176 -.005h11zm-10 3a1 1 0 0 0 -1 1v4l.007 .117a1 1 0 0 0 1.993 -.117v-4l-.007 -.117a1 1 0 0 0 -.993 -.883z"
        stroke - width = "0" fill = "currentColor" /> < title > { title } < / title > < /
        svg >
    }
}
