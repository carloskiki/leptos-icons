#[cfg(feature = "CgPentagonBottomLeft")]
use leptos::*;
#[cfg(feature = "CgPentagonBottomLeft")]
///This icon requires the feature `CgPentagonBottomLeft` to be enabled.
#[component]
pub fn PentagonBottomLeft(
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
        "M13.3301 9.23205L10 5L5 13.6603L8.33013 17.8923L13.6603 18.6603L18.6603 10L13.3301 9.23205ZM15.4512 11.5583L12.2531 11.0975L10.255 8.5583L7.40908 13.4876L9.40716 16.0268L12.6052 16.4876L15.4512 11.5583Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
