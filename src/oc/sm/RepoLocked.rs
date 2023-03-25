#[cfg(feature = "OcSmRepoLocked")]
use leptos::*;
#[cfg(feature = "OcSmRepoLocked")]
///This icon requires the feature `OcSmRepoLocked` to be enabled.
#[component]
pub fn RepoLocked(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M1 2.5A2.5 2.5 0 0 1 3.5 0h8.75a.75.75 0 0 1 .75.75v3.5a.75.75 0 0 1-1.5 0V1.5h-8a1 1 0 0 0-1 1v6.708A2.492 2.492 0 0 1 3.5 9h2.75a.75.75 0 0 1 0 1.5H3.5a1 1 0 1 0 0 2h2.75a.75.75 0 0 1 0 1.5H3.5A2.5 2.5 0 0 1 1 11.5v-9Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M9 10.168V9a3 3 0 1 1 6 0v1.168c.591.281 1 .884 1 1.582v2.5A1.75 1.75 0 0 1 14.25 16h-4.5A1.75 1.75 0 0 1 8 14.25v-2.5c0-.698.409-1.3 1-1.582ZM13.5 10V9a1.5 1.5 0 0 0-3 0v1Z"
        /> < title > { title } < / title > < / svg >
    }
}
