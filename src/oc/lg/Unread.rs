#[cfg(feature = "OcLgUnread")]
use leptos::*;
#[cfg(feature = "OcLgUnread")]
///This icon requires the feature `OcLgUnread` to be enabled.
#[component]
pub fn Unread(
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
        "M1.75 4.5a.25.25 0 0 0-.25.25v.852l10.36 7a.25.25 0 0 0 .28 0l5.69-3.845A.75.75 0 0 1 18.67 10l-5.69 3.845c-.592.4-1.368.4-1.96 0L1.5 7.412V19.25c0 .138.112.25.25.25h20.5a.25.25 0 0 0 .25-.25v-8.5a.75.75 0 0 1 1.5 0v8.5A1.75 1.75 0 0 1 22.25 21H1.75A1.75 1.75 0 0 1 0 19.25V4.75C0 3.784.784 3 1.75 3h15.5a.75.75 0 0 1 0 1.5H1.75Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M24 5.5a2.5 2.5 0 1 1-5 0 2.5 2.5 0 0 1 5 0Z" /> < title > { title } < / title >
        < / svg >
    }
}
