#[cfg(feature = "OcSmStrikethrough")]
use leptos::*;
#[cfg(feature = "OcSmStrikethrough")]
///This icon requires the feature `OcSmStrikethrough` to be enabled.
#[component]
pub fn Strikethrough(
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
        "M11.055 8.5c.524.536.815 1.257.811 2.007a3.133 3.133 0 0 1-1.12 2.408C9.948 13.597 8.748 14 7.096 14c-1.706 0-3.104-.607-3.902-1.377a.751.751 0 0 1 1.042-1.079c.48.463 1.487.956 2.86.956 1.422 0 2.232-.346 2.676-.726.435-.372.594-.839.594-1.267 0-.472-.208-.857-.647-1.197-.448-.346-1.116-.623-1.951-.81H1.75a.75.75 0 0 1 0-1.5h12.5a.75.75 0 0 1 0 1.5ZM7.581 3.25c-2.036 0-2.778 1.082-2.778 1.786 0 .055.002.107.006.157a.75.75 0 0 1-1.496.114 3.506 3.506 0 0 1-.01-.271c0-1.832 1.75-3.286 4.278-3.286 1.418 0 2.721.58 3.514 1.093a.75.75 0 1 1-.814 1.26c-.64-.414-1.662-.853-2.7-.853Z"
        /> < title > { title } < / title > < / svg >
    }
}
