#[cfg(feature = "VsLightbulb")]
use leptos::*;
#[cfg(feature = "VsLightbulb")]
///This icon requires the feature `VsLightbulb` to be enabled.
#[component]
pub fn Lightbulb(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M11.67 8.658a3.661 3.661 0 0 0-.781 1.114 3.28 3.28 0 0 0-.268 1.329v1.6a1.304 1.304 0 0 1-.794 1.197 1.282 1.282 0 0 1-.509.102H7.712a1.285 1.285 0 0 1-.922-.379 1.303 1.303 0 0 1-.38-.92v-1.6c0-.479-.092-.921-.274-1.329a3.556 3.556 0 0 0-.776-1.114 4.689 4.689 0 0 1-1.006-1.437A4.187 4.187 0 0 1 4 5.5a4.432 4.432 0 0 1 .616-2.27c.197-.336.432-.64.705-.914a4.6 4.6 0 0 1 .911-.702c.338-.196.7-.348 1.084-.454a4.45 4.45 0 0 1 1.2-.16 4.476 4.476 0 0 1 2.276.614 4.475 4.475 0 0 1 1.622 1.616 4.438 4.438 0 0 1 .616 2.27c0 .617-.117 1.191-.353 1.721a4.69 4.69 0 0 1-1.006 1.437zM9.623 10.5H7.409v2.201c0 .081.028.15.09.212a.29.29 0 0 0 .213.09h1.606a.289.289 0 0 0 .213-.09.286.286 0 0 0 .09-.212V10.5z"
        /> < title > { title } < / title > < / svg >
    }
}
