#[cfg(feature = "VsRootFolderOpened")]
use leptos::*;
#[cfg(feature = "VsRootFolderOpened")]
///This icon requires the feature `VsRootFolderOpened` to be enabled.
#[component]
pub fn RootFolderOpened(
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
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M1 6.257V2.5l.5-.5h5l.35.15.86.85h5.79l.5.5V6h1.13l.48.63-2.63 7-.48.37H8.743a5.48 5.48 0 0 0 .657-1h2.73l2.37-6H8.743a5.534 5.534 0 0 0-.72-.724l.127-.126L8.5 6H13V4H7.5l-.35-.15L6.29 3H2l.01 2.594c-.361.184-.7.407-1.01.663z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M6 10.5a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0z" />< path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M8 10.5a3.5 3.5 0 1 1-7 0 3.5 3.5 0 0 1 7 0zM4.5 13a2.5 2.5 0 1 0 0-5 2.5 2.5 0 0 0 0 5z"
        /> < title > { title } < / title > < / svg >
    }
}
