#[cfg(feature = "RiDeviceLineRadar")]
use leptos::*;
#[cfg(feature = "RiDeviceLineRadar")]
///This icon requires the feature `RiDeviceLineRadar` to be enabled.
#[component]
pub fn Radar(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M12.506 3.623l-1.023 1.772c-2.91-.879-5.514-.45-6.411 1.105-1.178 2.04.79 5.652 4.678 7.897s8 2.142 9.178.103c.898-1.555-.033-4.024-2.249-6.105l1.023-1.772c3.082 2.709 4.463 6.27 2.958 8.877-1.86 3.222-7.189 3.355-11.91.63C4.029 13.402 1.48 8.721 3.34 5.5c1.505-2.607 5.28-3.192 9.166-1.877zm3.378-1.85l1.732 1-5 8.66-1.732-1 5-8.66zM6.732 20H17v2H5.017a.995.995 0 0 1-.883-.5 1.005 1.005 0 0 1 0-1l2.25-3.897 1.732 1L6.732 20z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
