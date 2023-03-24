#[cfg(feature = "CgGitBranch")]
use leptos::*;
#[cfg(feature = "CgGitBranch")]
///This icon requires the feature `CgGitBranch` to be enabled.
#[component]
pub fn GitBranch(
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
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M9 3C7.89543 3 7 3.89543 7 5C7 5.74028 7.4022 6.38663 8 6.73244V17.2676C7.4022 17.6134 7 18.2597 7 19C7 20.1046 7.89543 21 9 21C9.74028 21 10.3866 20.5978 10.7324 20H11.9585C14.1676 20 15.9585 18.2091 15.9585 16V14.7324C16.5563 14.3866 16.9585 13.7403 16.9585 13C16.9585 11.8954 16.0631 11 14.9585 11C13.8539 11 12.9585 11.8954 12.9585 13C12.9585 13.7403 13.3607 14.3866 13.9585 14.7324V16C13.9585 17.1046 13.0631 18 11.9585 18H10.7324C10.5568 17.6964 10.3036 17.4432 10 17.2676V6.73244C10.5978 6.38663 11 5.74028 11 5C11 3.89543 10.1046 3 9 3Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
