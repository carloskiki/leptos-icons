#[cfg(feature = "WiMoonWaningGibbous5")]
use leptos::*;
#[cfg(feature = "WiMoonWaningGibbous5")]
///This icon requires the feature `WiMoonWaningGibbous5` to be enabled.
#[component]
pub fn MoonWaningGibbous5(
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
        "M3.74,14.47c0,2.03,0.5,3.91,1.51,5.63s2.37,3.09,4.09,4.09s3.6,1.51,5.63,1.51c2.17-2.75,3.25-6.5,3.25-11.24&#xA;	c0-3.96-1.08-7.71-3.25-11.24c-2.03,0-3.91,0.5-5.63,1.5S6.26,7.1,5.25,8.83S3.74,12.44,3.74,14.47z"
        /> < title > { title } < / title > < / svg >
    }
}
