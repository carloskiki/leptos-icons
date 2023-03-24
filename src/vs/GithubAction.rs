#[cfg(feature = "VsGithubAction")]
use leptos::*;
#[cfg(feature = "VsGithubAction")]
///This icon requires the feature `VsGithubAction` to be enabled.
#[component]
pub fn GithubAction(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M3.04 10h2.58l.65 1H2.54l-.5-.5v-9l.5-.5h12l.5.5v4.77l-1-1.75V2h-11v8zm5.54 1l-1.41 3.47h2.2L15 8.7 14.27 7h-1.63l.82-1.46L12.63 4H9.76l-.92.59-2.28 5L7.47 11h1.11zm1.18-6h2.87l-1.87 3h3.51l-5.76 5.84L10.2 10H7.47l2.29-5zM6.95 7H4.04V6H7.4l-.45 1zm-.9 2H4.04V8H6.5l-.45 1z"
        /> < title > { title } < / title > < / svg >
    }
}
