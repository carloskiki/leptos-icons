#[cfg(feature = "BiSolidMessageSquareDots")]
use leptos::*;
#[cfg(feature = "BiSolidMessageSquareDots")]
///This icon requires the feature `BiSolidMessageSquareDots` to be enabled.
#[component]
pub fn MessageSquareDots(
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
        "M16 2H8C4.691 2 2 4.691 2 8v13a1 1 0 0 0 1 1h13c3.309 0 6-2.691 6-6V8c0-3.309-2.691-6-6-6zm-5 10.5A1.5 1.5 0 0 1 9.5 14c-.086 0-.168-.011-.25-.025-.083.01-.164.025-.25.025a2 2 0 1 1 2-2c0 .085-.015.167-.025.25.013.082.025.164.025.25zm4 1.5c-.086 0-.167-.015-.25-.025a1.471 1.471 0 0 1-.25.025 1.5 1.5 0 0 1-1.5-1.5c0-.085.012-.168.025-.25-.01-.083-.025-.164-.025-.25a2 2 0 1 1 2 2z"
        /> < title > { title } < / title > < / svg >
    }
}
