#[cfg(feature = "TbSpadeFilled")]
use leptos::*;
#[cfg(feature = "TbSpadeFilled")]
///This icon requires the feature `TbSpadeFilled` to be enabled.
#[component]
pub fn SpadeFilled(
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
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-spade-filled" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M11.327 2.26a1395.065 1395.065 0 0 0 -4.923 4.504c-.626 .6 -1.212 1.21 -1.774 1.843a6.528 6.528 0 0 0 -.314 8.245l.14 .177c1.012 1.205 2.561 1.755 4.055 1.574l.246 -.037l-.706 2.118a1 1 0 0 0 .949 1.316h6l.118 -.007a1 1 0 0 0 .83 -1.31l-.688 -2.065l.104 .02c1.589 .25 3.262 -.387 4.32 -1.785a6.527 6.527 0 0 0 -.311 -8.243a31.787 31.787 0 0 0 -1.76 -1.83l-4.938 -4.518a1 1 0 0 0 -1.348 -.001z"
        stroke - width = "0" fill = "currentColor" /> < title > { title } < / title > < /
        svg >
    }
}
