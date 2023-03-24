#[cfg(feature = "OcLgBookmarkSlash")]
use leptos::*;
#[cfg(feature = "OcLgBookmarkSlash")]
///This icon requires the feature `OcLgBookmarkSlash` to be enabled.
#[component]
pub fn BookmarkSlash(
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
        "M1.565 2.018v-.001l21.75 15.75a.75.75 0 1 1-.88 1.215L19 16.495v4.764a.748.748 0 0 1-1.219.584L12 17.21l-5.781 4.634A.75.75 0 0 1 5 21.259V6.357L.685 3.232a.75.75 0 0 1 .88-1.214ZM17.5 15.408l-11-7.965v12.254l5.031-4.032a.749.749 0 0 1 .938 0l5.031 4.032ZM7.25 2a.75.75 0 0 0 0 1.5h10a.25.25 0 0 1 .25.25v6.5a.75.75 0 0 0 1.5 0v-6.5A1.75 1.75 0 0 0 17.25 2h-10Z"
        /> < title > { title } < / title > < / svg >
    }
}
