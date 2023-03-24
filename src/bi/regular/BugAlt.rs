#[cfg(feature = "BiRegularBugAlt")]
use leptos::*;
#[cfg(feature = "BiRegularBugAlt")]
///This icon requires the feature `BiRegularBugAlt` to be enabled.
#[component]
pub fn BugAlt(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 9h6v2H9zm0 4h6v2H9z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "m18 5.414 1.707-1.707-1.414-1.414-1.563 1.562C15.483 2.708 13.824 2 12 2s-3.483.708-4.73 1.855L5.707 2.293 4.293 3.707 6 5.414A6.937 6.937 0 0 0 5 9H3v2h2v2H3v2h2c0 3.86 3.141 7 7 7s7-3.14 7-7h2v-2h-2v-2h2V9h-2a6.937 6.937 0 0 0-1-3.586zM17 13v2c0 2.757-2.243 5-5 5s-5-2.243-5-5V9c0-2.757 2.243-5 5-5s5 2.243 5 5v4z"
        /> < title > { title } < / title > < / svg >
    }
}
