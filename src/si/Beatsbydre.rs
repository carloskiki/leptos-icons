#[cfg(feature = "SiBeatsbydre")]
use leptos::*;
#[cfg(feature = "SiBeatsbydre")]
///This icon requires the feature `SiBeatsbydre` to be enabled.
#[component]
pub fn Beatsbydre(
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
        "M12.0099 15.5996A3.5995 3.5995 0 1 0 8.4103 12a3.5995 3.5995 0 0 0 3.5996 3.5996zm0-15.5981a11.9985 11.9985 0 0 0-3.5996.552v6.6471A5.9992 5.9992 0 1 1 6.0106 12V1.6033A11.9985 11.9985 0 1 0 12.01.0015z"
        /> < title > { title } < / title > < / svg >
    }
}
