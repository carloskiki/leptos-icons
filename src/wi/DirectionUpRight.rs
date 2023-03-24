#[cfg(feature = "WiDirectionUpRight")]
use leptos::*;
#[cfg(feature = "WiDirectionUpRight")]
///This icon requires the feature `WiDirectionUpRight` to be enabled.
#[component]
pub fn DirectionUpRight(
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
        "M10.05,17.55c0,0.3,0.09,0.55,0.26,0.73c0.2,0.19,0.46,0.28,0.79,0.28c0.3,0,0.55-0.09,0.73-0.28l6.04-6.05v1.95&#xA;	c0,0.3,0.1,0.55,0.3,0.75c0.2,0.2,0.45,0.3,0.75,0.3c0.29,0,0.54-0.1,0.74-0.31s0.3-0.45,0.3-0.75V9.7c0-0.3-0.1-0.55-0.3-0.75&#xA;	s-0.45-0.3-0.74-0.3h-4.5c-0.29,0-0.54,0.1-0.73,0.3S13.4,9.39,13.4,9.7c0,0.29,0.1,0.54,0.29,0.73s0.44,0.29,0.73,0.29h1.95&#xA;	l-6.06,6.06C10.14,16.99,10.05,17.25,10.05,17.55z"
        /> < title > { title } < / title > < / svg >
    }
}
