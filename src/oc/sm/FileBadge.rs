#[cfg(feature = "OcSmFileBadge")]
use leptos::*;
#[cfg(feature = "OcSmFileBadge")]
///This icon requires the feature `OcSmFileBadge` to be enabled.
#[component]
pub fn FileBadge(
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
        width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M2.75 1.5a.25.25 0 0 0-.25.25v11.5c0 .138.112.25.25.25h3.5a.75.75 0 0 1 0 1.5h-3.5A1.75 1.75 0 0 1 1 13.25V1.75C1 .784 1.784 0 2.75 0h8a1.75 1.75 0 0 1 1.508.862.75.75 0 1 1-1.289.768.25.25 0 0 0-.219-.13h-8Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 7a3.999 3.999 0 0 1 7.605-1.733 4 4 0 0 1-1.115 4.863l.995 4.973a.75.75 0 0 1-.991.852l-2.409-.876a.248.248 0 0 0-.17 0l-2.409.876a.75.75 0 0 1-.991-.852l.994-4.973A3.994 3.994 0 0 1 8 7Zm4-2.5a2.5 2.5 0 1 0 0 5 2.5 2.5 0 0 0 0-5Zm0 6.5c-.373 0-.745-.051-1.104-.154l-.649 3.243 1.155-.42c.386-.14.81-.14 1.196 0l1.155.42-.649-3.243A4.004 4.004 0 0 1 12 11Z"
        /> < title > { title } < / title > < / svg >
    }
}
