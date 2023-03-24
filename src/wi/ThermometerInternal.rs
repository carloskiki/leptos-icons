#[cfg(feature = "WiThermometerInternal")]
use leptos::*;
#[cfg(feature = "WiThermometerInternal")]
///This icon requires the feature `WiThermometerInternal` to be enabled.
#[component]
pub fn ThermometerInternal(
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
        stroke_witdh = "0" style = style version = "1.1" id = "Layer_1" x = "0px" y =
        "0px" viewBox = "0 0 30 30" style = "enable-background:new 0 0 30 30;" space =
        "preserve" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" xmlns : xlink = "http://www.w3.org/1999/xlink" d =
        "M12.48,19.56c0,0.71,0.24,1.32,0.73,1.82c0.49,0.5,1.07,0.75,1.76,0.75s1.28-0.25,1.79-0.75c0.51-0.5,0.76-1.11,0.76-1.81&#xA;	c0-0.63-0.22-1.19-0.65-1.67c-0.43-0.48-0.96-0.77-1.57-0.85V9.69c0-0.06-0.03-0.13-0.1-0.19c-0.07-0.07-0.14-0.1-0.22-0.1&#xA;	c-0.09,0-0.16,0.03-0.21,0.08c-0.05,0.06-0.08,0.12-0.08,0.21v7.34c-0.61,0.09-1.13,0.37-1.56,0.85&#xA;	C12.69,18.37,12.48,18.92,12.48,19.56z"
        /> < title > { title } < / title > < / svg >
    }
}
