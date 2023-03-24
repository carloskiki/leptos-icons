#[cfg(feature = "OcLgRepoTemplate")]
use leptos::*;
#[cfg(feature = "OcLgRepoTemplate")]
///This icon requires the feature `OcLgRepoTemplate` to be enabled.
#[component]
pub fn RepoTemplate(
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
        "M5.75 0A2.75 2.75 0 0 0 3 2.75v1a.75.75 0 0 0 1.5 0v-1c0-.69.56-1.25 1.25-1.25h1a.75.75 0 0 0 0-1.5h-1Zm4 0a.75.75 0 0 0 0 1.5h4.5a.75.75 0 0 0 0-1.5h-4.5Zm7.5 0a.75.75 0 0 0 0 1.5h2.25v2.25a.75.75 0 0 0 1.5 0v-3a.75.75 0 0 0-.75-.75h-3ZM4.5 6.5a.75.75 0 0 0-1.5 0v3.75a.75.75 0 0 0 1.5 0V6.5Zm16.5 0a.75.75 0 0 0-1.5 0v3.75a.75.75 0 0 0 1.5 0V6.5ZM4.5 13.25a.75.75 0 0 0-1.5 0v5.5a3.25 3.25 0 0 0 1.95 2.98.75.75 0 1 0 .6-1.375A1.75 1.75 0 0 1 4.5 18.75V18A1.5 1.5 0 0 1 6 16.5h.75a.75.75 0 0 0 0-1.5H6c-.546 0-1.059.146-1.5.401V13.25Zm16.5 0a.75.75 0 0 0-1.5 0V15h-2.25a.75.75 0 0 0 0 1.5h2.25v4h-5.25a.75.75 0 0 0 0 1.5h6a.75.75 0 0 0 .75-.75v-8ZM9.75 15a.75.75 0 0 0 0 1.5h4.5a.75.75 0 0 0 0-1.5h-4.5Zm-2.353 8.461A.25.25 0 0 1 7 23.26v-5.01a.25.25 0 0 1 .25-.25h5a.25.25 0 0 1 .25.25v5.01a.25.25 0 0 1-.397.201l-2.206-1.604a.25.25 0 0 0-.294 0L7.397 23.46Z"
        /> < title > { title } < / title > < / svg >
    }
}
