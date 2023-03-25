#[cfg(feature = "TbDeviceNintendo")]
use leptos::*;
#[cfg(feature = "TbDeviceNintendo")]
///This icon requires the feature `TbDeviceNintendo` to be enabled.
#[component]
pub fn DeviceNintendo(
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
        "icon icon-tabler icon-tabler-device-nintendo" width = "24" height = "24" viewBox
        = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10 20v-16h-3a4 4 0 0 0 -4 4v8a4 4 0 0 0 4 4h3z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M14 20v-16h3a4 4 0 0 1 4 4v8a4 4 0 0 1 -4 4h-3z" />< circle xmlns =
        "http://www.w3.org/2000/svg" cx = "17.5" cy = "15.5" r = "1" fill =
        "currentColor" />< circle xmlns = "http://www.w3.org/2000/svg" cx = "6.5" cy =
        "8.5" r = "1" fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
