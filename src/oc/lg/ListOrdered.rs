#[cfg(feature = "OcLgListOrdered")]
use leptos::*;
#[cfg(feature = "OcLgListOrdered")]
///This icon requires the feature `OcLgListOrdered` to be enabled.
#[component]
pub fn ListOrdered(
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
        "M3.604 3.089A.75.75 0 0 1 4 3.75V8.5h.75a.75.75 0 0 1 0 1.5h-3a.75.75 0 1 1 0-1.5h.75V5.151l-.334.223a.75.75 0 0 1-.832-1.248l1.5-1a.75.75 0 0 1 .77-.037ZM8.75 5.5a.75.75 0 0 0 0 1.5h11.5a.75.75 0 0 0 0-1.5H8.75Zm0 6a.75.75 0 0 0 0 1.5h11.5a.75.75 0 0 0 0-1.5H8.75Zm0 6a.75.75 0 0 0 0 1.5h11.5a.75.75 0 0 0 0-1.5H8.75ZM5.5 15.75c0-.704-.271-1.286-.72-1.686a2.302 2.302 0 0 0-1.53-.564c-.535 0-1.094.178-1.53.565-.449.399-.72.982-.72 1.685a.75.75 0 0 0 1.5 0c0-.296.104-.464.217-.564A.805.805 0 0 1 3.25 15c.215 0 .406.072.533.185.113.101.217.268.217.565 0 .332-.069.48-.21.657-.092.113-.216.24-.403.419l-.147.14c-.152.144-.33.313-.52.504l-1.5 1.5a.75.75 0 0 0-.22.53v.25c0 .414.336.75.75.75H5A.75.75 0 0 0 5 19H3.31l.47-.47c.176-.176.333-.324.48-.465l.165-.156a5.98 5.98 0 0 0 .536-.566c.358-.447.539-.925.539-1.593Z"
        /> < title > { title } < / title > < / svg >
    }
}
