#[cfg(feature = "WiRefreshAlt")]
use leptos::*;
#[cfg(feature = "WiRefreshAlt")]
///This icon requires the feature `WiRefreshAlt` to be enabled.
#[component]
pub fn RefreshAlt(
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
        "M11.78,14.91c0,0.79,0.19,1.51,0.57,2.17c0.38,0.66,0.9,1.19,1.57,1.57c0.67,0.38,1.39,0.58,2.18,0.58&#xA;	c0.19,0,0.35-0.07,0.48-0.22c0.13-0.14,0.2-0.31,0.2-0.51c0-0.19-0.07-0.35-0.2-0.48s-0.29-0.19-0.49-0.19&#xA;	c-0.81,0-1.5-0.28-2.07-0.85c-0.57-0.57-0.85-1.26-0.85-2.07c0-0.78,0.27-1.45,0.8-2.02s1.16-0.86,1.88-0.86l-0.33,0.32&#xA;	c-0.15,0.15-0.22,0.31-0.21,0.49c0,0.18,0.07,0.34,0.2,0.48c0.13,0.14,0.29,0.21,0.49,0.21c0.2,0,0.37-0.07,0.51-0.21l1.51-1.5&#xA;	c0.13-0.11,0.2-0.27,0.2-0.51c0-0.22-0.07-0.38-0.2-0.47l-1.51-1.53c-0.13-0.14-0.29-0.21-0.49-0.21s-0.36,0.07-0.5,0.21&#xA;	s-0.21,0.3-0.21,0.5c0,0.21,0.07,0.38,0.22,0.51l0.3,0.28c-1.15,0.08-2.11,0.53-2.89,1.35C12.17,12.77,11.78,13.76,11.78,14.91z"
        /> < title > { title } < / title > < / svg >
    }
}
