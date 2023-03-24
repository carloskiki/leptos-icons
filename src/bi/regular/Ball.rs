#[cfg(feature = "BiRegularBall")]
use leptos::*;
#[cfg(feature = "BiRegularBall")]
///This icon requires the feature `BiRegularBall` to be enabled.
#[component]
pub fn Ball(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M3.76 21a17.68 17.68 0 0 0 4 .43 13.89 13.89 0 0 0 9.93-3.69C23 12.37 21.06 4.11 21 3.76a1 1 0 0 0-.76-.76 17.91 17.91 0 0 0-4-.43 13.82 13.82 0 0 0-9.96 3.71C.94 11.63 2.94 19.89 3 20.24a1 1 0 0 0 .76.76zM7.7 7.7a11.86 11.86 0 0 1 8.49-3.1 17.57 17.57 0 0 1 3 .25c.31 1.87.91 7.67-2.86 11.44a11.91 11.91 0 0 1-8.55 3.11 17.16 17.16 0 0 1-2.93-.25c-.32-1.88-.92-7.67 2.85-11.45z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "m7.29 15.29 1.42 1.42 1.79-1.79 1.79 1.79 1.42-1.42-1.8-1.79 1.59-1.59 1.79 1.8 1.42-1.42-1.8-1.79 1.8-1.79-1.42-1.42-1.79 1.8-1.79-1.8-1.42 1.42 1.8 1.79-1.59 1.59-1.79-1.8-1.42 1.42 1.8 1.79z"
        /> < title > { title } < / title > < / svg >
    }
}
