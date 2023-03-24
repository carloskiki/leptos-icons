#[cfg(feature = "IoCashSharp")]
use leptos::*;
#[cfg(feature = "IoCashSharp")]
///This icon requires the feature `IoCashSharp` to be enabled.
#[component]
pub fn CashSharp(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = { size.clone() } height = { size } > < rect xmlns =
        "http://www.w3.org/2000/svg" x = "48" y = "368" width = "416" height = "32" /><
        rect xmlns = "http://www.w3.org/2000/svg" x = "80" y = "416" width = "352" height
        = "32" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M480,176a96.11,96.11,0,0,1-96-96V64H128V80a96.11,96.11,0,0,1-96,96H16v64H32a96.11,96.11,0,0,1,96,96v16H384V336a96.11,96.11,0,0,1,96-96h16V176ZM256,304a96,96,0,1,1,96-96A96.11,96.11,0,0,1,256,304Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M96,80V64H16v80H32A64.07,64.07,0,0,0,96,80Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M32,272H16v80H96V336A64.07,64.07,0,0,0,32,272Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M480,144h16V64H416V80A64.07,64.07,0,0,0,480,144Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M416,336v16h80V272H480A64.07,64.07,0,0,0,416,336Z" />< circle xmlns =
        "http://www.w3.org/2000/svg" cx = "256" cy = "208" r = "64" /> < title > { title
        } < / title > < / svg >
    }
}
