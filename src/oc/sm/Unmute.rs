#[cfg(feature = "OcSmUnmute")]
use leptos::*;
#[cfg(feature = "OcSmUnmute")]
///This icon requires the feature `OcSmUnmute` to be enabled.
#[component]
pub fn Unmute(
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
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M7.563 2.069A.75.75 0 0 1 8 2.75v10.5a.751.751 0 0 1-1.238.57L3.472 11H1.75A1.75 1.75 0 0 1 0 9.25v-2.5C0 5.784.784 5 1.75 5h1.723l3.289-2.82a.75.75 0 0 1 .801-.111ZM6.5 4.38 4.238 6.319a.748.748 0 0 1-.488.181h-2a.25.25 0 0 0-.25.25v2.5c0 .138.112.25.25.25h2c.179 0 .352.064.488.18L6.5 11.62Zm6.096-2.038a.75.75 0 0 1 1.06 0 8 8 0 0 1 0 11.314.751.751 0 0 1-1.042-.018.751.751 0 0 1-.018-1.042 6.5 6.5 0 0 0 0-9.193.75.75 0 0 1 0-1.06Zm-1.06 2.121-.001.001a5 5 0 0 1 0 7.07.749.749 0 0 1-1.275-.326.749.749 0 0 1 .215-.734 3.5 3.5 0 0 0 0-4.95.75.75 0 1 1 1.061-1.061Z"
        /> < title > { title } < / title > < / svg >
    }
}
