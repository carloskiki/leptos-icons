#[cfg(feature = "VsGitCommit")]
use leptos::*;
#[cfg(feature = "VsGitCommit")]
///This icon requires the feature `VsGitCommit` to be enabled.
#[component]
pub fn GitCommit(
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
        "http://www.w3.org/2000/svg" d =
        "M7.5 0H8.5V4.03095C10.4732 4.277 12 5.96019 12 8C12 10.0398 10.4732 11.723 8.5 11.9691V16H7.5V11.9691C5.52684 11.723 4 10.0398 4 8C4 5.96019 5.52684 4.277 7.5 4.03095V0ZM8 10.6C9.43594 10.6 10.6 9.43594 10.6 8C10.6 6.56406 9.43594 5.4 8 5.4C6.56406 5.4 5.4 6.56406 5.4 8C5.4 9.43594 6.56406 10.6 8 10.6Z"
        /> < title > { title } < / title > < / svg >
    }
}
