#[cfg(feature = "WiMoonWaningGibbous6")]
use leptos::*;
#[cfg(feature = "WiMoonWaningGibbous6")]
///This icon requires the feature `WiMoonWaningGibbous6` to be enabled.
#[component]
pub fn MoonWaningGibbous6(
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
        stroke_witdh = "0" style = style version = "1.1" id = "Layer_1" x = "0px" y =
        "0px" viewBox = "0 0 30 30" style = "enable-background:new 0 0 30 30;" space =
        "preserve" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" d =
        "M3.74,14.46c0,2.04,0.5,3.92,1.51,5.65s2.37,3.09,4.09,4.09s3.61,1.51,5.65,1.51c1.44-3.08,2.15-6.83,2.15-11.25&#xA;	c0-3.46-0.72-7.2-2.15-11.24c-1.52,0-2.98,0.3-4.37,0.89S8.03,5.5,7.03,6.5s-1.8,2.2-2.4,3.59S3.74,12.93,3.74,14.46z"
        /> < title > { title } < / title > < / svg >
    }
}
