#[cfg(feature = "TbLungsOff")]
use leptos::*;
#[cfg(feature = "TbLungsOff")]
///This icon requires the feature `TbLungsOff` to be enabled.
#[component]
pub fn LungsOff(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-lungs-off"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M6.583 6.608c-1.206 1.058 -2.07 2.626 -2.933 5.449c-.42 1.37 -.636 2.962 -.648 4.775c-.012 1.675 1.261 3.054 2.877 3.161l.203 .007c1.611 0 2.918 -1.335 2.918 -2.98v-8.02"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15 11v-3.743c0 -.694 .552 -1.257 1.233 -1.257c.204 0 .405 .052 .584 .15l.13 .083c1.46 1.059 2.432 2.647 3.405 5.824c.42 1.37 .636 2.962 .648 4.775c0 .063 0 .125 0 .187m-1.455 2.51c-.417 .265 -.9 .43 -1.419 .464l-.202 .007c-1.613 0 -2.92 -1.335 -2.92 -2.98v-2.02"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M9 12a2.99 2.99 0 0 0 2.132 -.89" />< path xmlns = "http://www.w3.org/2000/svg"
        d = "M12 4v4" />< path xmlns = "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> <
        title > { title } < / title > < / svg >
    }
}
