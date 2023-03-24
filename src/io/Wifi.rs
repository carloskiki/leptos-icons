#[cfg(feature = "IoWifi")]
use leptos::*;
#[cfg(feature = "IoWifi")]
///This icon requires the feature `IoWifi` to be enabled.
#[component]
pub fn Wifi(
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
        "0 0 512 512" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M346.65,304.3a136,136,0,0,0-180.71,0,21,21,0,1,0,27.91,31.38,94,94,0,0,1,124.89,0,21,21,0,0,0,27.91-31.4Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M256.28,183.7a221.47,221.47,0,0,0-151.8,59.92,21,21,0,1,0,28.68,30.67,180.28,180.28,0,0,1,246.24,0,21,21,0,1,0,28.68-30.67A221.47,221.47,0,0,0,256.28,183.7Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M462,175.86a309,309,0,0,0-411.44,0,21,21,0,1,0,28,31.29,267,267,0,0,1,355.43,0,21,21,0,0,0,28-31.31Z"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "256.28" cy = "393.41" r =
        "32" /> < title > { title } < / title > < / svg >
    }
}
