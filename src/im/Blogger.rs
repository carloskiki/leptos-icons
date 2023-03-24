#[cfg(feature = "ImBlogger")]
use leptos::*;
#[cfg(feature = "ImBlogger")]
///This icon requires the feature `ImBlogger` to be enabled.
#[component]
pub fn Blogger(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M14.966 6h-0.897c-0.549 0-1.031-0.465-1.069-1v0c0-2.854-2.301-5-5.175-5h-2.622c-2.872 0-5.201 2.313-5.203 5.167v5.669c0 2.854 2.331 5.165 5.203 5.165h5.6c2.874 0 5.197-2.311 5.197-5.165v-3.662c0-0.57-0.46-1.173-1.034-1.173zM5 4h3c0.55 0 1 0.45 1 1s-0.45 1-1 1h-3c-0.55 0-1-0.45-1-1s0.45-1 1-1zM11 12h-6c-0.55 0-1-0.45-1-1s0.45-1 1-1h6c0.55 0 1 0.45 1 1s-0.45 1-1 1z"
        /> < title > { title } < / title > < / svg >
    }
}
