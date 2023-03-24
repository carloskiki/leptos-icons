#[cfg(feature = "IoNotificationsOffOutline")]
use leptos::*;
#[cfg(feature = "IoNotificationsOffOutline")]
///This icon requires the feature `IoNotificationsOffOutline` to be enabled.
#[component]
pub fn NotificationsOffOutline(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M128.51,204.59q-.37,6.15-.37,12.76C128.14,304,110,320,84.33,351.43,73.69,364.45,83,384,101.62,384H320"
        style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M414.5,335.3c-18.48-23.45-30.62-47.05-30.62-118C383.88,138,343.36,109.73,310,96c-4.43-1.82-8.6-6-9.95-10.55C294.21,65.54,277.82,48,256,48S217.8,65.55,212,85.47c-1.35,4.6-5.52,8.71-10,10.53a149.57,149.57,0,0,0-18,8.79"
        style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M320,384v16a64,64,0,0,1-128,0V384" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "448" y1 = "448" x2 = "64" y2
        = "64" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px"
        /> < title > { title } < / title > < / svg >
    }
}
