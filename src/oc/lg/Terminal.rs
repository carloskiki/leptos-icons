#[cfg(feature = "OcLgTerminal")]
use leptos::*;
#[cfg(feature = "OcLgTerminal")]
///This icon requires the feature `OcLgTerminal` to be enabled.
#[component]
pub fn Terminal(
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
        "M9.25 12a.75.75 0 0 1-.22.53l-2.75 2.75a.75.75 0 0 1-1.06-1.06L7.44 12 5.22 9.78a.75.75 0 1 1 1.06-1.06l2.75 2.75c.141.14.22.331.22.53Zm2 2a.75.75 0 0 0 0 1.5h5a.75.75 0 0 0 0-1.5h-5Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M0 4.75C0 3.784.784 3 1.75 3h20.5c.966 0 1.75.784 1.75 1.75v14.5A1.75 1.75 0 0 1 22.25 21H1.75A1.75 1.75 0 0 1 0 19.25Zm1.75-.25a.25.25 0 0 0-.25.25v14.5c0 .138.112.25.25.25h20.5a.25.25 0 0 0 .25-.25V4.75a.25.25 0 0 0-.25-.25Z"
        /> < title > { title } < / title > < / svg >
    }
}
