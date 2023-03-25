#[cfg(feature = "TiDeviceLaptop")]
use leptos::*;
#[cfg(feature = "TiDeviceLaptop")]
///This icon requires the feature `TiDeviceLaptop` to be enabled.
#[component]
pub fn DeviceLaptop(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M21.989 16.049c.009-.315.011-.657.011-1.049v-9c0-1.654-1.346-3-3-3h-14c-1.654 0-3 1.346-3 3v9c0 .385.002.73.012 1.049-1.145.228-2.012 1.24-2.012 2.451 0 1.378 1.122 2.5 2.5 2.5h19c1.378 0 2.5-1.122 2.5-2.5 0-1.211-.866-2.222-2.011-2.451zm-17.989-10.049c0-.551.449-1 1-1h14c.551 0 1 .449 1 1v9c0 .388-.005.726-.014 1h-.986v-9c0-.55-.45-1-1-1h-12c-.55 0-1 .45-1 1v9h-.98c-.012-.264-.02-.599-.02-1v-9zm14 10h-12v-9h12v9zm3.5 3h-19c-.271 0-.5-.229-.5-.5s.229-.5.5-.5h19c.271 0 .5.229.5.5s-.229.5-.5.5z"
        /> < title > { title } < / title > < / svg >
    }
}
