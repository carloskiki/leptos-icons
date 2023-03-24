#[cfg(feature = "AiOutlinedFundView")]
use leptos::*;
#[cfg(feature = "AiOutlinedFundView")]
///This icon requires the feature `AiOutlinedFundView` to be enabled.
#[component]
pub fn FundView(
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
        stroke_witdh = "0" style = style t = "1569683816874" class = "icon" viewBox =
        "0 0 1024 1024" version = "1.1" p - id = "14256" width = "200" height = "200"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < defs
        xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" >< style type = "text/css" /></ defs >< path xmlns
        = "http://www.w3.org/2000/svg" xmlns : xlink = "http://www.w3.org/1999/xlink" d =
        "M956 686.5l-0.1-0.1-0.1-0.1C911.7 593 843.4 545 752.5 545s-159.2 48.1-203.4 141.3v0.1c-5.4 11.5-5.4 24.9 0 36.4C593.3 816 661.6 864 752.5 864s159.2-48.1 203.4-141.3c5.4-11.5 5.4-24.8 0.1-36.2zM752.5 800c-62.1 0-107.4-30-141.1-95.5C645 639 690.4 609 752.5 609c62.1 0 107.4 30 141.1 95.5C860 770 814.6 800 752.5 800z"
        p - id = "14257" />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" d =
        "M753 705m-56 0a56 56 0 1 0 112 0 56 56 0 1 0-112 0Z" p - id = "14258" />< path
        xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" d =
        "M136 232h704v253h72V192c0-17.7-14.3-32-32-32H96c-17.7 0-32 14.3-32 32v520c0 17.7 14.3 32 32 32h352v-72H136V232z"
        p - id = "14259" />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" d =
        "M724.9 338.1l-36.8-36.8c-3.1-3.1-8.2-3.1-11.3 0L493 485.3l-86.1-86.2c-3.1-3.1-8.2-3.1-11.3 0L251.3 543.4c-3.1 3.1-3.1 8.2 0 11.3l36.8 36.8c3.1 3.1 8.2 3.1 11.3 0l101.8-101.8 86.1 86.2c3.1 3.1 8.2 3.1 11.3 0l226.3-226.5c3.2-3.1 3.2-8.2 0-11.3z"
        p - id = "14260" /> < title > { title } < / title > < / svg >
    }
}
