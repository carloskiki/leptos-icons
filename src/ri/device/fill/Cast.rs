#[cfg(feature = "RiDeviceFillCast")]
use leptos::*;
#[cfg(feature = "RiDeviceFillCast")]
///This icon requires the feature `RiDeviceFillCast` to be enabled.
#[component]
pub fn Cast(
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
        "M3 3h18a1 1 0 0 1 1 1v16a1 1 0 0 1-1 1h-6a13.1 13.1 0 0 0-.153-2H20V5H4v3.153A13.1 13.1 0 0 0 2 8V4a1 1 0 0 1 1-1zm10 18h-2a9 9 0 0 0-9-9v-2c6.075 0 11 4.925 11 11zm-4 0H7a5 5 0 0 0-5-5v-2a7 7 0 0 1 7 7zm-4 0H2v-3a3 3 0 0 1 3 3zm9.373-4A13.032 13.032 0 0 0 6 8.627V7h12v10h-3.627z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
