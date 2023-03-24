#[cfg(feature = "FaSolidSquareUpRight")]
use leptos::*;
#[cfg(feature = "FaSolidSquareUpRight")]
///This icon requires the feature `FaSolidSquareUpRight` to be enabled.
#[component]
pub fn SquareUpRight(
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
        stroke_witdh = "0" style = style viewBox = "0 0 448 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M384 32c35.3 0 64 28.7 64 64V416c0 35.3-28.7 64-64 64H64c-35.3 0-64-28.7-64-64V96C0 60.7 28.7 32 64 32H384zM320 313.4V176c0-8.8-7.2-16-16-16H166.6c-12.5 0-22.6 10.1-22.6 22.6c0 6 2.4 11.8 6.6 16L184 232l-66.3 66.3C114 302 112 306.9 112 312s2 10 5.7 13.7l36.7 36.7c3.6 3.6 8.5 5.7 13.7 5.7s10-2 13.7-5.7L248 296l33.4 33.4c4.2 4.2 10 6.6 16 6.6c12.5 0 22.6-10.1 22.6-22.6z"
        /> < title > { title } < / title > < / svg >
    }
}
