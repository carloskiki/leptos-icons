#[cfg(feature = "TbGitBranchDeleted")]
use leptos::*;
#[cfg(feature = "TbGitBranchDeleted")]
///This icon requires the feature `TbGitBranchDeleted` to be enabled.
#[component]
pub fn GitBranchDeleted(
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
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-git-branch-deleted" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M7 18m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M7 6m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M7 8v8" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 18h6a2 2 0 0 0 2 -2v-5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14 14l3 -3l3 3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 4l4 4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 8l4 -4" /> < title > { title } < / title >
        < / svg >
    }
}
