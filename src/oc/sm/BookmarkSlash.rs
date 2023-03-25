#[cfg(feature = "OcSmBookmarkSlash")]
use leptos::*;
#[cfg(feature = "OcSmBookmarkSlash")]
///This icon requires the feature `OcSmBookmarkSlash` to be enabled.
#[component]
pub fn BookmarkSlash(
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
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M1.19 1.143 4.182 3.31l.014.01 8.486 6.145.014.01 2.994 2.168a.75.75 0 1 1-.88 1.214L13 11.547v2.703a.75.75 0 0 1-1.206.596L8 11.944l-3.794 2.902A.75.75 0 0 1 3 14.25V4.305L.31 2.357a.75.75 0 1 1 .88-1.214ZM4.5 5.39v7.341l3.044-2.328a.75.75 0 0 1 .912 0l3.044 2.328V10.46ZM5.865 1h5.385c.966 0 1.75.784 1.75 1.75v3.624a.75.75 0 0 1-1.5 0V2.75a.25.25 0 0 0-.25-.25H5.865a.75.75 0 0 1 0-1.5Z"
        /> < title > { title } < / title > < / svg >
    }
}
