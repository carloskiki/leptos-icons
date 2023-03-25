#[cfg(feature = "TiPuzzle")]
use leptos::*;
#[cfg(feature = "TiPuzzle")]
///This icon requires the feature `TiPuzzle` to be enabled.
#[component]
pub fn Puzzle(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M16.25 11.25c.364 0 .704.145.984.391.549.332.766-.034.766-.391v-1.75c0-.825-.675-1.5-1.5-1.5h-2.75c-.356 0-.724-.216-.391-.766.246-.28.391-.619.391-.984 0-.967-1.007-1.75-2.25-1.75s-2.25.783-2.25 1.75c0 .3.095.576.255.823.507.673.136.927-.255.927h-2.75c-.825 0-1.5.675-1.5 1.5v1.75c0 .391.254.762.928.244.246-.149.522-.244.822-.244.966 0 1.75 1.008 1.75 2.25s-.784 2.25-1.75 2.25c-.364 0-.704-.145-.984-.391-.549-.332-.766.034-.766.391v2.75c0 .825.675 1.5 1.5 1.5h2.75c.391 0 .762-.254.243-.927-.148-.247-.243-.523-.243-.823 0-.967 1.007-1.75 2.25-1.75s2.25.783 2.25 1.75c0 .365-.145.704-.391.984-.333.55.035.766.391.766h2.75c.825 0 1.5-.675 1.5-1.5v-2.75c0-.391-.254-.762-.928-.244-.246.149-.522.244-.822.244-.966 0-1.75-1.008-1.75-2.25s.784-2.25 1.75-2.25z"
        /> < title > { title } < / title > < / svg >
    }
}
