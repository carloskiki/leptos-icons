#[cfg(feature = "SiYoast")]
use leptos::*;
#[cfg(feature = "SiYoast")]
///This icon requires the feature `SiYoast` to be enabled.
#[component]
pub fn Yoast(
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
        "M15.864 0L14.55 3.652H5.422A3.592 3.592 0 0 0 1.84 7.233v9.529a3.592 3.592 0 0 0 3.582 3.581h1.495a4.9 4.9 0 0 1-.18.029l-.34.047V24h.391c2.76 0 4.442-1.385 5.706-3.657h9.666V7.233a3.593 3.593 0 0 0-3.253-3.565L20.275 0zm.556.778h2.738l-6.055 16.22c-1.55 4.335-3.186 6.064-5.924 6.21v-2.12c1.767-.354 2.418-1.461 2.785-2.408a3.902 3.902 0 0 0 0-2.828L6.43 6.772h2.488l2.512 7.86z"
        /> < title > { title } < / title > < / svg >
    }
}
