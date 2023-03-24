#[cfg(feature = "TbDeviceWatchOff")]
use leptos::*;
#[cfg(feature = "TbDeviceWatchOff")]
///This icon requires the feature `TbDeviceWatchOff` to be enabled.
#[component]
pub fn DeviceWatchOff(
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
        "icon icon-tabler icon-tabler-device-watch-off" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d
        = "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10 6h5a3 3 0 0 1 3 3v5m-.882 3.125a2.99 2.99 0 0 1 -2.118 .875h-6a3 3 0 0 1 -3 -3v-6c0 -.828 .336 -1.578 .878 -2.121"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M9 18v3h6v-3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 5v-2h6v3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
