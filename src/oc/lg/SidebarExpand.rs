#[cfg(feature = "OcLgSidebarExpand")]
use leptos::*;
#[cfg(feature = "OcLgSidebarExpand")]
///This icon requires the feature `OcLgSidebarExpand` to be enabled.
#[component]
pub fn SidebarExpand(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M11.28 9.53 8.81 12l2.47 2.47a.749.749 0 0 1-.326 1.275.749.749 0 0 1-.734-.215l-3-3a.75.75 0 0 1 0-1.06l3-3a.749.749 0 0 1 1.275.326.749.749 0 0 1-.215.734Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3.75 2h16.5c.966 0 1.75.784 1.75 1.75v16.5A1.75 1.75 0 0 1 20.25 22H3.75A1.75 1.75 0 0 1 2 20.25V3.75C2 2.784 2.784 2 3.75 2ZM3.5 3.75v16.5c0 .138.112.25.25.25H15v-17H3.75a.25.25 0 0 0-.25.25Zm13 16.75h3.75a.25.25 0 0 0 .25-.25V3.75a.25.25 0 0 0-.25-.25H16.5Z"
        /> < title > { title } < / title > < / svg >
    }
}
