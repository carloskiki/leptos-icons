#[cfg(feature = "VsRepoForcePush")]
use leptos::*;
#[cfg(feature = "VsRepoForcePush")]
///This icon requires the feature `VsRepoForcePush` to be enabled.
#[component]
pub fn RepoForcePush(
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
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M3.74 1h9.76l.5.5v12l-.5.5H10v-1h3v-2h-3v-1h3V2H4v8h3v1H3.74a.74.74 0 0 0-.74.75v.5a.74.74 0 0 0 .74.75H7v1H3.74A1.74 1.74 0 0 1 2 12.25v-9.5A1.74 1.74 0 0 1 3.74 1zm1.6 4.83l.71.7L8 4.58v1.45L5.38 8.65l.71.7 1.92-1.92V15h1V7.328l2.03 2.022.7-.7L9 5.9V4.538l2 1.992.7-.7L8.88 3h-.71L5.34 5.83z"
        /> < title > { title } < / title > < / svg >
    }
}
