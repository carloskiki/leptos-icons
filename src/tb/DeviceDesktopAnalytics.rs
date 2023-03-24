#[cfg(feature = "TbDeviceDesktopAnalytics")]
use leptos::*;
#[cfg(feature = "TbDeviceDesktopAnalytics")]
///This icon requires the feature `TbDeviceDesktopAnalytics` to be enabled.
#[component]
pub fn DeviceDesktopAnalytics(
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
        "icon icon-tabler icon-tabler-device-desktop-analytics" width = "24" height =
        "24" viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill =
        "none" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3 4m0 1a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v10a1 1 0 0 1 -1 1h-16a1 1 0 0 1 -1 -1z"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M7 20h10" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 16v4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 16v4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 12v-4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 12v-1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 12v-2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 12v-1" /> < title > { title } < / title > <
        / svg >
    }
}
