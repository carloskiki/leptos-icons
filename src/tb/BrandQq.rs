#[cfg(feature = "TbBrandQq")]
use leptos::*;
#[cfg(feature = "TbBrandQq")]
///This icon requires the feature `TbBrandQq` to be enabled.
#[component]
pub fn BrandQq(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-brand-qq"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M6 9.748a14.716 14.716 0 0 0 11.995 -.052c.275 -9.236 -11.104 -11.256 -11.995 .052z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M18 10c.984 2.762 1.949 4.765 2 7.153c.014 .688 -.664 1.346 -1.184 .303c-.346 -.696 -.952 -1.181 -1.816 -1.456"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M17 16c.031 1.831 .147 3.102 -1 4" />< path xmlns = "http://www.w3.org/2000/svg"
        d = "M8 20c-1.099 -.87 -.914 -2.24 -1 -4" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M6 10c-.783 2.338 -1.742 4.12 -1.968 6.43c-.217 2.227 .716 1.644 1.16 .917c.296 -.487 .898 -.934 1.808 -1.347"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M15.898 13l-.476 -2" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M8 20l-1.5 1c-.5 .5 -.5 1 .5 1h10c1 0 1 -.5 .5 -1l-1.5 -1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M13.75 7m-1 0a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M10.25 7m-1 0a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> < title > { title } < / title > <
        / svg >
    }
}
