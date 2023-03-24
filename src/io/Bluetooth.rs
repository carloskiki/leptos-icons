#[cfg(feature = "IoBluetooth")]
use leptos::*;
#[cfg(feature = "IoBluetooth")]
///This icon requires the feature `IoBluetooth` to be enabled.
#[component]
pub fn Bluetooth(
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
        "M388,160.77a20,20,0,0,0-5.85-14.91l-112-112A20,20,0,0,0,236,48V212.52l-79-67.71a20,20,0,0,0-26,30.38L225.27,256,131,336.81a20,20,0,1,0,26,30.38l79-67.71V464a20,20,0,0,0,34.14,14.14l112-112A20,20,0,0,0,381,336.81L286.73,256,381,175.19A20,20,0,0,0,388,160.77ZM338.58,353.13,276,415.72V299.49ZM276,212.52V96.28l62.59,62.59Z"
        /> < title > { title } < / title > < / svg >
    }
}
