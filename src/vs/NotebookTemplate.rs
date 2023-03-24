#[cfg(feature = "VsNotebookTemplate")]
use leptos::*;
#[cfg(feature = "VsNotebookTemplate")]
///This icon requires the feature `VsNotebookTemplate` to be enabled.
#[component]
pub fn NotebookTemplate(
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
        "M1 5H0V4h1v1zm0 2H0V6h1v1zm0 2H0V8h1v1zm0 2H0v-1h1v1zm0 2H0v-1h1v1zm0 1v1H0v-1h1zm0 1h1v1H1v-1zm2 0h1v1H3v-1zM1 1H0V0h1v1zm2 0H2V0h1v1zm1-1h1v1H4V0zm3 1H6V0h1v1zm2 0H8V0h1v1zm2 0h-1V0h1v1zm0 1V1h1v1h-1zm1 2h-1V3h1v1zM1 3H0V2h1v1z"
        />< path xmlns = "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule
        = "evenodd" d = "M5 6l1-1h7l1 1v9l-1 1H6l-1-1V6zm1 0v9h7V6H6z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M15 7h1v2h-1V7zm0 3h1v2h-1v-2zm0 3h1v2h-1v-2zM7 8h5v1H7z" /> < title > { title }
        < / title > < / svg >
    }
}
