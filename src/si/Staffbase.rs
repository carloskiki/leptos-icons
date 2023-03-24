#[cfg(feature = "SiStaffbase")]
use leptos::*;
#[cfg(feature = "SiStaffbase")]
///This icon requires the feature `SiStaffbase` to be enabled.
#[component]
pub fn Staffbase(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M11.847 20.095a7.805 7.805 0 01-6.286-3.238l1.714-1.238C8.323 17.048 10.037 18 11.847 18s3.523-.857 4.571-2.381l1.714 1.238a7.805 7.805 0 01-6.285 3.238zm.19-18c1.62 0 3.238.476 4.762 1.334l1.048.476 2.857-.572-.477 2.857c2.381 3.715 2.191 9.239-1.047 12.667a9.748 9.748 0 01-7.048 3.048 9.98 9.98 0 01-6.857-2.762c-3.905-3.81-4-10-.286-13.905 1.905-2.095 4.477-3.143 7.048-3.143m0-2.095C8.799 0 5.751 1.333 3.466 3.714c-4.572 4.762-4.477 12.381.285 16.953A11.91 11.91 0 0012.037 24c3.238 0 6.381-1.333 8.571-3.619 3.62-3.714 4.286-9.81 1.81-14.571l.38-2.096.477-2.952-2.952.571-2.19.381-.382-.19C15.941.476 14.037 0 12.037 0Z"
        /> < title > { title } < / title > < / svg >
    }
}
