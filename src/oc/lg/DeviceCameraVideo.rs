#[cfg(feature = "OcLgDeviceCameraVideo")]
use leptos::*;
#[cfg(feature = "OcLgDeviceCameraVideo")]
///This icon requires the feature `OcLgDeviceCameraVideo` to be enabled.
#[component]
pub fn DeviceCameraVideo(
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
        "M24 5.25v13a.75.75 0 0 1-1.136.643L16.5 15.075v2.175A1.75 1.75 0 0 1 14.75 19h-13A1.75 1.75 0 0 1 0 17.25v-11C0 5.284.784 4.5 1.75 4.5h13c.966 0 1.75.784 1.75 1.75v2.175l6.364-3.818A.75.75 0 0 1 24 5.25Zm-9 1a.25.25 0 0 0-.25-.25h-13a.25.25 0 0 0-.25.25v11c0 .138.112.25.25.25h13a.25.25 0 0 0 .25-.25v-11Zm1.5 7.075 6 3.6V6.575l-6 3.6Z"
        /> < title > { title } < / title > < / svg >
    }
}
