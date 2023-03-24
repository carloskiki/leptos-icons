#[cfg(feature = "CgArrowLongDownC")]
use leptos::*;
#[cfg(feature = "CgArrowLongDownC")]
///This icon requires the feature `CgArrowLongDownC` to be enabled.
#[component]
pub fn ArrowLongDownC(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M10.9991 6.84976C9.83339 6.43819 8.99813 5.32671 8.99813 4.02014C8.99813 2.36329 10.3413 1.02014 11.9981 1.02014C13.655 1.02014 14.9981 2.36329 14.9981 4.02014C14.9981 5.32601 14.1638 6.43701 12.9991 6.84911L13.0121 19.1375L14.8244 17.315L16.2426 18.7253L12.0119 22.9799L7.75739 18.7491L9.16763 17.3309L11.0122 19.1652L10.9991 6.84976ZM11.9981 5.02014C11.4458 5.02014 10.9981 4.57243 10.9981 4.02014C10.9981 3.46786 11.4458 3.02014 11.9981 3.02014C12.5504 3.02014 12.9981 3.46786 12.9981 4.02014C12.9981 4.57243 12.5504 5.02014 11.9981 5.02014Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
