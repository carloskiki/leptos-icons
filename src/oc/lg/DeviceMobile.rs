#[cfg(feature = "OcLgDeviceMobile")]
use leptos::*;
#[cfg(feature = "OcLgDeviceMobile")]
///This icon requires the feature `OcLgDeviceMobile` to be enabled.
#[component]
pub fn DeviceMobile(
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
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M10.25 5.25A.75.75 0 0 1 11 4.5h2A.75.75 0 0 1 13 6h-2a.75.75 0 0 1-.75-.75ZM12 19.5a1 1 0 1 0 0-2 1 1 0 0 0 0 2Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M4 2.75C4 1.784 4.784 1 5.75 1h12.5c.966 0 1.75.784 1.75 1.75v18.5A1.75 1.75 0 0 1 18.25 23H5.75A1.75 1.75 0 0 1 4 21.25Zm1.75-.25a.25.25 0 0 0-.25.25v18.5c0 .138.112.25.25.25h12.5a.25.25 0 0 0 .25-.25V2.75a.25.25 0 0 0-.25-.25Z"
        /> < title > { title } < / title > < / svg >
    }
}
