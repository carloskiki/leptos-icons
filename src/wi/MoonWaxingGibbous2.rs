#[cfg(feature = "WiMoonWaxingGibbous2")]
use leptos::*;
#[cfg(feature = "WiMoonWaxingGibbous2")]
///This icon requires the feature `WiMoonWaxingGibbous2` to be enabled.
#[component]
pub fn MoonWaxingGibbous2(
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
        "M12.85,14.44c0,4.77,0.71,8.52,2.14,11.26c2.04,0,3.93-0.5,5.65-1.51s3.1-2.37,4.1-4.1s1.51-3.61,1.51-5.65&#xA;	s-0.5-3.92-1.51-5.65s-2.37-3.09-4.1-4.09s-3.61-1.51-5.65-1.51C13.57,6.61,12.85,10.36,12.85,14.44z"
        /> < title > { title } < / title > < / svg >
    }
}
