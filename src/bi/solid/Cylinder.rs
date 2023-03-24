#[cfg(feature = "BiSolidCylinder")]
use leptos::*;
#[cfg(feature = "BiSolidCylinder")]
///This icon requires the feature `BiSolidCylinder` to be enabled.
#[component]
pub fn Cylinder(
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
        "M12 22c5.131 0 9-1.935 9-4.5V7c0-.051-.024-.097-.033-.146.016-.117.033-.234.033-.354C21 3.935 17.131 2 12 2S3 3.935 3 6.5v11c0 2.565 3.869 4.5 9 4.5zm0-18c4.273 0 7 1.48 7 2.5a.683.683 0 0 1-.025.158c-.004.01-.012.018-.015.027-.274.848-2.29 1.98-5.496 2.253l-.05.003C12.965 8.979 12.494 9 12 9 7.727 9 5 7.52 5 6.5S7.727 4 12 4z"
        /> < title > { title } < / title > < / svg >
    }
}
