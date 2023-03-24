#[cfg(feature = "IoLogoMedium")]
use leptos::*;
#[cfg(feature = "IoLogoMedium")]
///This icon requires the feature `IoLogoMedium` to be enabled.
#[component]
pub fn LogoMedium(
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
        stroke_witdh = "0" style = style version = "1.1" x = "0px" y = "0px" viewBox =
        "0 0 512 512" style = "enable-background:new 0 0 512 512;" space = "preserve"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < g
        xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" id = "boxes" style = "display:none;" />< g xmlns =
        "http://www.w3.org/2000/svg" xmlns : xlink = "http://www.w3.org/1999/xlink" id =
        "icons" >< path d =
        "M28,28v456h456V28H28z M406.83,136.04l-24.46,23.45c-2.11,1.61-3.15,4.25-2.72,6.86v172.28c-0.44,2.61,0.61,5.26,2.72,6.86&#xA;		l23.88,23.45v5.15H286.13v-5.15l24.74-24.02c2.43-2.43,2.43-3.15,2.43-6.86V198.81l-68.79,174.71h-9.3l-80.09-174.71v117.1&#xA;		c-0.67,4.92,0.97,9.88,4.43,13.44l32.18,39.03v5.15h-91.24v-5.15l32.18-39.03c3.44-3.57,4.98-8.56,4.15-13.44V180.5&#xA;		c0.38-3.76-1.05-7.48-3.86-10.01l-28.6-34.46v-5.15h88.81l68.65,150.55l60.35-150.55h84.66V136.04z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
