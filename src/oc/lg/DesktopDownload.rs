#[cfg(feature = "OcLgDesktopDownload")]
use leptos::*;
#[cfg(feature = "OcLgDesktopDownload")]
///This icon requires the feature `OcLgDesktopDownload` to be enabled.
#[component]
pub fn DesktopDownload(
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
        "M11.25 9.331V.75a.75.75 0 0 1 1.5 0v8.58l1.949-2.11A.75.75 0 1 1 15.8 8.237l-3.25 3.52a.75.75 0 0 1-1.102 0l-3.25-3.52A.75.75 0 1 1 9.3 7.22l1.949 2.111Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M2.5 3.75v11.5c0 .138.112.25.25.25h18.5a.25.25 0 0 0 .25-.25V3.75a.25.25 0 0 0-.25-.25h-5.5a.75.75 0 0 1 0-1.5h5.5c.966 0 1.75.784 1.75 1.75v11.5A1.75 1.75 0 0 1 21.25 17h-6.204c.171 1.375.805 2.652 1.769 3.757A.752.752 0 0 1 16.25 22h-8.5a.75.75 0 0 1-.566-1.243c.965-1.105 1.599-2.382 1.77-3.757H2.75A1.75 1.75 0 0 1 1 15.25V3.75C1 2.784 1.784 2 2.75 2h5.5a.75.75 0 0 1 0 1.5h-5.5a.25.25 0 0 0-.25.25ZM10.463 17c-.126 1.266-.564 2.445-1.223 3.5h5.52c-.66-1.055-1.098-2.234-1.223-3.5Z"
        /> < title > { title } < / title > < / svg >
    }
}
