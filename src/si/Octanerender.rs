#[cfg(feature = "SiOctanerender")]
use leptos::*;
#[cfg(feature = "SiOctanerender")]
///This icon requires the feature `SiOctanerender` to be enabled.
#[component]
pub fn Octanerender(
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
        "M11.71 0C8.24 3.9 6.92 6 6.64 9.14c-.01-.01-.03-.01-.04-.02-1.28-.73-2.3-2.22-2.91-3.73l-2.23.87c1.64 4.95 2.81 7.13 5.39 8.94-.02.01-.03.02-.05.03-1.27.74-3.07.89-4.68.66l-.36 2.37c5.11 1.06 7.59 1.15 10.46-.19v.06c0 1.47-.77 3.09-1.78 4.38L12.3 24c3.46-3.89 4.78-5.99 5.06-9.13.02.01.03.01.05.02 1.27.73 2.29 2.21 2.9 3.73l2.23-.87c-1.64-4.95-2.8-7.14-5.39-8.95.02-.01.03-.02.05-.03 1.27-.74 3.07-.88 4.68-.65l.36-2.38c-5.1-1.06-7.58-1.14-10.44.19v-.06c0-1.47.77-3.09 1.78-4.38L11.71 0zm.19 8.82a3.181 3.181 0 0 1 3.28 3.07 3.181 3.181 0 0 1-3.07 3.28 3.181 3.181 0 0 1-3.28-3.07 3.181 3.181 0 0 1 3.07-3.28z"
        /> < title > { title } < / title > < / svg >
    }
}
