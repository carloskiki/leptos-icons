#[cfg(feature = "WiMoonWaningCrescent5")]
use leptos::*;
#[cfg(feature = "WiMoonWaningCrescent5")]
///This icon requires the feature `WiMoonWaningCrescent5` to be enabled.
#[component]
pub fn MoonWaningCrescent5(
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
        "M3.74,14.44c0,2.04,0.5,3.93,1.51,5.65s2.37,3.1,4.09,4.1s3.61,1.51,5.65,1.51c-2.59-0.79-4.48-2.13-5.69-4.02&#xA;	s-1.81-4.3-1.81-7.24c0-1.39,0.2-2.7,0.6-3.95c0.4-1.25,0.94-2.34,1.63-3.27s1.48-1.75,2.37-2.44s1.86-1.22,2.89-1.59&#xA;	c-2.04,0-3.92,0.5-5.65,1.51S6.26,7.07,5.25,8.8S3.74,12.4,3.74,14.44z"
        /> < title > { title } < / title > < / svg >
    }
}
