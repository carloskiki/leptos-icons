#[cfg(feature = "ImTv")]
use leptos::*;
#[cfg(feature = "ImTv")]
///This icon requires the feature `ImTv` to be enabled.
#[component]
pub fn Tv(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M15.331 4.502c-1.388-0.199-2.865-0.344-4.407-0.425l2.576-2.576-1-1-3.509 3.509c-0.328-0.006-0.659-0.009-0.991-0.009v0l-4-4-1 1 3.034 3.034c-1.889 0.066-3.693 0.227-5.365 0.467-0.43 1.683-0.669 3.543-0.669 5.498s0.239 3.815 0.669 5.498c2.244 0.323 4.724 0.502 7.331 0.502s5.087-0.179 7.331-0.502c0.43-1.683 0.669-3.543 0.669-5.498s-0.239-3.815-0.669-5.498zM13.498 13.666c-1.683 0.215-3.543 0.334-5.498 0.334s-3.815-0.119-5.498-0.334c-0.323-1.122-0.502-2.362-0.502-3.666s0.179-2.543 0.502-3.666c1.683-0.215 3.543-0.334 5.498-0.334s3.815 0.119 5.498 0.334c0.323 1.122 0.502 2.362 0.502 3.666s-0.179 2.543-0.502 3.666z"
        /> < title > { title } < / title > < / svg >
    }
}
