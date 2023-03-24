#[cfg(feature = "WiFahrenheit")]
use leptos::*;
#[cfg(feature = "WiFahrenheit")]
///This icon requires the feature `WiFahrenheit` to be enabled.
#[component]
pub fn Fahrenheit(
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
        "M9.67,11.01c0-0.5,0.18-0.93,0.53-1.28s0.78-0.53,1.28-0.53c0.49,0,0.92,0.18,1.27,0.53c0.35,0.36,0.53,0.78,0.53,1.28&#xA;	c0,0.5-0.18,0.93-0.53,1.29c-0.35,0.36-0.78,0.54-1.27,0.54s-0.92-0.18-1.28-0.54C9.85,11.94,9.67,11.51,9.67,11.01z M10.55,11.01&#xA;	c0,0.26,0.09,0.48,0.27,0.67c0.19,0.19,0.41,0.28,0.67,0.28s0.48-0.09,0.67-0.28s0.28-0.41,0.28-0.67c0-0.26-0.09-0.48-0.28-0.66&#xA;	c-0.19-0.18-0.41-0.28-0.67-0.28c-0.26,0-0.48,0.09-0.67,0.27C10.64,10.52,10.55,10.74,10.55,11.01z M14.96,17.9&#xA;	c0,0.14,0.05,0.27,0.15,0.37s0.23,0.15,0.37,0.15c0.14,0,0.27-0.05,0.37-0.15c0.1-0.1,0.15-0.23,0.15-0.37v-3.79h2.86&#xA;	c0.14,0,0.27-0.05,0.37-0.16s0.15-0.23,0.15-0.38c0-0.15-0.05-0.27-0.15-0.38c-0.1-0.1-0.23-0.15-0.38-0.15h-2.86v-2.73h3.82&#xA;	c0.14,0,0.26-0.05,0.36-0.15s0.14-0.23,0.14-0.38s-0.05-0.27-0.14-0.38s-0.21-0.15-0.36-0.15h-4.77c-0.07,0-0.1,0.04-0.1,0.11V17.9z&#xA;	"
        /> < title > { title } < / title > < / svg >
    }
}
