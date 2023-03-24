#[cfg(feature = "OcLgRepoLocked")]
use leptos::*;
#[cfg(feature = "OcLgRepoLocked")]
///This icon requires the feature `OcLgRepoLocked` to be enabled.
#[component]
pub fn RepoLocked(
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
        "http://www.w3.org/2000/svg" d =
        "M2 2.75A2.75 2.75 0 0 1 4.75 0h14.5a.75.75 0 0 1 .75.75v8a.75.75 0 0 1-1.5 0V1.5H4.75c-.69 0-1.25.56-1.25 1.25v12.651A2.987 2.987 0 0 1 5 15h6.25a.75.75 0 0 1 0 1.5H5A1.5 1.5 0 0 0 3.5 18v1.25c0 .69.56 1.25 1.25 1.25h6a.75.75 0 0 1 0 1.5h-6A2.75 2.75 0 0 1 2 19.25V2.75Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15 14.5a3.5 3.5 0 1 1 7 0V16h.25c.966 0 1.75.784 1.75 1.75v4.5A1.75 1.75 0 0 1 22.25 24h-7.5A1.75 1.75 0 0 1 13 22.25v-4.5c0-.966.784-1.75 1.75-1.75H15Zm3.5-2a2 2 0 0 0-2 2V16h4v-1.5a2 2 0 0 0-2-2Z"
        /> < title > { title } < / title > < / svg >
    }
}
