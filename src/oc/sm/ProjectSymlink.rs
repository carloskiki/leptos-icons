#[cfg(feature = "OcSmProjectSymlink")]
use leptos::*;
#[cfg(feature = "OcSmProjectSymlink")]
///This icon requires the feature `OcSmProjectSymlink` to be enabled.
#[component]
pub fn ProjectSymlink(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M0 1.75C0 .784.784 0 1.75 0h12.5C15.216 0 16 .784 16 1.75v12.5A1.75 1.75 0 0 1 14.25 16h-8.5a.75.75 0 0 1 0-1.5h8.5a.25.25 0 0 0 .25-.25V6.5h-13v1.75a.75.75 0 0 1-1.5 0ZM6.5 5h8V1.75a.25.25 0 0 0-.25-.25H6.5Zm-5 0H5V1.5H1.75a.25.25 0 0 0-.25.25Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M1.5 13.737a2.25 2.25 0 0 1 2.262-2.25L4 11.49v1.938c0 .218.26.331.42.183l2.883-2.677a.25.25 0 0 0 0-.366L4.42 7.89a.25.25 0 0 0-.42.183V9.99l-.23-.001A3.75 3.75 0 0 0 0 13.738v1.012a.75.75 0 0 0 1.5 0v-1.013Z"
        /> < title > { title } < / title > < / svg >
    }
}
