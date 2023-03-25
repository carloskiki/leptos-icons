#[cfg(feature = "IoColorWand")]
use leptos::*;
#[cfg(feature = "IoColorWand")]
///This icon requires the feature `IoColorWand` to be enabled.
#[component]
pub fn ColorWand(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style version = "1.1" id = "icons" x = "0px" y = "0px"
        viewBox = "0 0 512 512" style = "enable-background:new 0 0 512 512;" space =
        "preserve" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" d =
        "M96,208H48c-8.8,0-16-7.2-16-16s7.2-16,16-16h48c8.8,0,16,7.2,16,16S104.8,208,96,208z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" d =
        "M124.1,140.1c-4.2,0-8.3-1.7-11.3-4.7l-33.9-33.9c-6.2-6.2-6.2-16.4,0-22.6s16.4-6.2,22.6,0l33.9,33.9&#xA;	c6.3,6.2,6.3,16.4,0,22.6C132.4,138.4,128.4,140.1,124.1,140.1z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" d =
        "M192,112c-8.8,0-16-7.2-16-16V48c0-8.8,7.2-16,16-16s16,7.2,16,16v48C208,104.8,200.8,112,192,112z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" d =
        "M259.9,140.1c-8.8,0-16-7.2-16-16c0-4.2,1.7-8.3,4.7-11.3l33.9-33.9c6.2-6.2,16.4-6.2,22.6,0c6.2,6.2,6.2,16.4,0,22.6&#xA;	l-33.9,33.9C268.2,138.4,264.1,140.1,259.9,140.1z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" d =
        "M90.2,309.8c-8.8,0-16-7.2-16-16c0-4.2,1.7-8.3,4.7-11.3l33.9-33.9c6.2-6.2,16.4-6.2,22.6,0s6.2,16.4,0,22.6l-33.9,33.9&#xA;	C98.5,308.1,94.4,309.8,90.2,309.8z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" d =
        "M234.2,167c-18.4-18.7-48.5-19-67.2-0.7s-19,48.5-0.7,67.2c0.2,0.2,0.5,0.5,0.7,0.7l39.5,39.5c3.1,3.1,8.2,3.1,11.3,0&#xA;	l55.9-55.9c3.1-3.1,3.1-8.2,0-11.3L234.2,167z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" d =
        "M457,389.8L307.6,240.4c-3.1-3.1-8.2-3.1-11.3,0l-55.9,55.9c-3.1,3.1-3.1,8.2,0,11.3L389.8,457c18.4,18.7,48.5,19,67.2,0.7&#xA;	c18.7-18.4,19-48.5,0.7-67.2C457.5,390.3,457.3,390,457,389.8L457,389.8z"
        /> < title > { title } < / title > < / svg >
    }
}
