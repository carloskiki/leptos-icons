#[cfg(feature = "OcLgGitPullRequestClosed")]
use leptos::*;
#[cfg(feature = "OcLgGitPullRequestClosed")]
///This icon requires the feature `OcLgGitPullRequestClosed` to be enabled.
#[component]
pub fn GitPullRequestClosed(
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
        "M22.266 2.711a.75.75 0 1 0-1.061-1.06l-1.983 1.983-1.984-1.983a.75.75 0 1 0-1.06 1.06l1.983 1.983-1.983 1.984a.75.75 0 0 0 1.06 1.06l1.984-1.983 1.983 1.983a.75.75 0 0 0 1.06-1.06l-1.983-1.984 1.984-1.983ZM4.75 1.5a3.25 3.25 0 0 1 .745 6.414A.827.827 0 0 1 5.5 8v8a.827.827 0 0 1-.005.086A3.25 3.25 0 0 1 4.75 22.5a3.25 3.25 0 0 1-.745-6.414A.827.827 0 0 1 4 16V8c0-.029.002-.057.005-.086A3.25 3.25 0 0 1 4.75 1.5ZM16 19.25a3.252 3.252 0 0 1 2.5-3.163V9.625a.75.75 0 0 1 1.5 0v6.462a3.252 3.252 0 0 1-.75 6.413A3.25 3.25 0 0 1 16 19.25ZM3 4.75a1.75 1.75 0 1 0 3.501-.001A1.75 1.75 0 0 0 3 4.75Zm0 14.5a1.75 1.75 0 1 0 3.501-.001A1.75 1.75 0 0 0 3 19.25Zm16.25-1.75a1.75 1.75 0 1 0 .001 3.501 1.75 1.75 0 0 0-.001-3.501Z"
        /> < title > { title } < / title > < / svg >
    }
}
