#[cfg(feature = "SiCoveralls")]
use leptos::*;
#[cfg(feature = "SiCoveralls")]
///This icon requires the feature `SiCoveralls` to be enabled.
#[component]
pub fn Coveralls(
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
        "M0 12v12h24V0H0zm13.195-6.187l1.167 3.515 2.255.005c1.238.005 2.916.019 3.727.037l1.472.028-2.968 2.152c-1.63 1.181-2.976 2.18-2.99 2.212-.01.033.487 1.627 1.106 3.54.619 1.917 1.12 3.487 1.116 3.492-.005.01-1.35-.947-2.986-2.119-1.636-1.177-3-2.147-3.033-2.161-.028-.01-1.411.947-3.07 2.138-1.655 1.185-3.02 2.151-3.024 2.142-.004-.005.497-1.575 1.116-3.492.619-1.913 1.115-3.507 1.106-3.54-.014-.032-1.36-1.03-2.99-2.212L2.23 9.398l1.472-.028c.811-.018 2.49-.032 3.727-.037l2.254-.005 1.168-3.515a512.54 512.54 0 011.171-3.516c.005 0 .53 1.58 1.172 3.516z"
        /> < title > { title } < / title > < / svg >
    }
}
