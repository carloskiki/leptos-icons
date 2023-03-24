#[cfg(feature = "VsDashboard")]
use leptos::*;
#[cfg(feature = "VsDashboard")]
///This icon requires the feature `VsDashboard` to be enabled.
#[component]
pub fn Dashboard(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M3.889 2.095a6.5 6.5 0 1 1 7.222 10.81A6.5 6.5 0 0 1 3.89 2.094zm.555 9.978A5.5 5.5 0 0 0 7.5 13 5.506 5.506 0 0 0 13 7.5a5.5 5.5 0 1 0-8.556 4.573zM10.294 4l.706.707-2.15 2.15a1.514 1.514 0 1 1-.707-.707L10.293 4zM7.221 7.916a.5.5 0 1 0 .556-.832.5.5 0 0 0-.556.832zm4.286-2.449l-.763.763c.166.403.253.834.255 1.27a3.463 3.463 0 0 1-.5 1.777l.735.735a4.477 4.477 0 0 0 .274-4.545h-.001zM8.733 4.242A3.373 3.373 0 0 0 7.5 4 3.5 3.5 0 0 0 4 7.5a3.46 3.46 0 0 0 .5 1.777l-.734.735A4.5 4.5 0 0 1 9.5 3.473l-.767.769z"
        /> < title > { title } < / title > < / svg >
    }
}
