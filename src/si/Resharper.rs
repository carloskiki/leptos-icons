#[cfg(feature = "SiResharper")]
use leptos::*;
#[cfg(feature = "SiResharper")]
///This icon requires the feature `SiResharper` to be enabled.
#[component]
pub fn Resharper(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M24 0H0v24h24V0ZM3.006 19.476h9v1.5h-9v-1.5Zm9.824-9.392h-.98V8.432h1.264l.324-1.908h-1.136V4.872h1.416l.324-1.864h1.704l-.332 1.864h1.88l.324-1.864h1.7l-.304 1.864h.98v1.652h-1.28l-.324 1.908h1.136v1.652h-1.42l-.336 1.936h-1.7l.332-1.936h-1.868l-.32 1.936h-1.72l.336-1.936ZM3.014 3.008H7.15c1.144 0 2.021.304 2.632.912.529.553.808 1.3.772 2.064v.028a2.802 2.802 0 0 1-.536 1.784 3.065 3.065 0 0 1-1.404.992l2.204 3.224h-2.32L6.63 9.24H5.014v2.772h-2V3.008Zm13.672 5.424.328-1.908h-1.876l-.32 1.908h1.868ZM7.014 7.408c.415.014.821-.129 1.136-.4a1.2 1.2 0 0 0 .4-.928v-.028a1.156 1.156 0 0 0-.4-.968 1.836 1.836 0 0 0-1.148-.32H5.014v2.592l2 .052Z"
        /> < title > { title } < / title > < / svg >
    }
}
