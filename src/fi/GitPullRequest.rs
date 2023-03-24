#[cfg(feature = "FiGitPullRequest")]
use leptos::*;
#[cfg(feature = "FiGitPullRequest")]
///This icon requires the feature `FiGitPullRequest` to be enabled.
#[component]
pub fn GitPullRequest(
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
        fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap =
        "round" stroke - linejoin = "round" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < circle xmlns = "http://www.w3.org/2000/svg" cx =
        "18" cy = "18" r = "3" />< circle xmlns = "http://www.w3.org/2000/svg" cx = "6"
        cy = "6" r = "3" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M13 6h3a2 2 0 0 1 2 2v7" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "6"
        y1 = "9" x2 = "6" y2 = "21" /> < title > { title } < / title > < / svg >
    }
}
