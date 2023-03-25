#[cfg(feature = "BiLogosSquarespace")]
use leptos::*;
#[cfg(feature = "BiLogosSquarespace")]
///This icon requires the feature `BiLogosSquarespace` to be enabled.
#[component]
pub fn Squarespace(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M20.886 9.264c-1.502-1.5-3.941-1.5-5.475 0l-6.13 6.131a.981.981 0 0 0 0 1.377.982.982 0 0 0 1.376 0l6.13-6.131a1.925 1.925 0 0 1 2.722 2.723l-6.037 6.035c.751.75 1.971.75 2.723 0l4.66-4.66c1.531-1.534 1.531-3.973.031-5.475zm-2.064 2.033a.982.982 0 0 0-1.376 0l-6.134 6.162c-.751.75-1.97.75-2.72 0-.376-.375-1.002-.375-1.377 0s-.375 1.002 0 1.375c1.502 1.502 3.942 1.502 5.475 0l6.131-6.129c.375-.406.375-1.033.001-1.408zm-2.035-6.129c-1.501-1.502-3.941-1.502-5.475 0l-6.131 6.129a.98.98 0 0 0 0 1.375.98.98 0 0 0 1.377 0l6.168-6.129a1.934 1.934 0 0 1 2.727 0 .982.982 0 0 0 1.376 0c.352-.406.352-1-.024-1.375h-.018zm-2.063 2.031a.978.978 0 0 0-1.376 0l-6.131 6.164a1.933 1.933 0 0 1-2.723 0 1.927 1.927 0 0 1 0-2.723l6.037-6.039a1.928 1.928 0 0 0-2.722 0L3.118 9.264c-1.501 1.502-1.501 3.941 0 5.475 1.501 1.502 3.94 1.502 5.475 0l6.129-6.131c.377-.408.377-1.033 0-1.408h.002z"
        /> < title > { title } < / title > < / svg >
    }
}
