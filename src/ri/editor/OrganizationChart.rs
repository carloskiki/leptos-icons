#[cfg(feature = "RiEditorOrganizationChart")]
use leptos::*;
#[cfg(feature = "RiEditorOrganizationChart")]
///This icon requires the feature `RiEditorOrganizationChart` to be enabled.
#[component]
pub fn OrganizationChart(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0H24V24H0z" />< path d
        =
        "M15 3c.552 0 1 .448 1 1v4c0 .552-.448 1-1 1h-2v2h4c.552 0 1 .448 1 1v3h2c.552 0 1 .448 1 1v4c0 .552-.448 1-1 1h-6c-.552 0-1-.448-1-1v-4c0-.552.448-1 1-1h2v-2H8v2h2c.552 0 1 .448 1 1v4c0 .552-.448 1-1 1H4c-.552 0-1-.448-1-1v-4c0-.552.448-1 1-1h2v-3c0-.552.448-1 1-1h4V9H9c-.552 0-1-.448-1-1V4c0-.552.448-1 1-1h6zM9 17H5v2h4v-2zm10 0h-4v2h4v-2zM14 5h-4v2h4V5z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
