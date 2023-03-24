#[cfg(feature = "VsGraph")]
use leptos::*;
#[cfg(feature = "VsGraph")]
///This icon requires the feature `VsGraph` to be enabled.
#[component]
pub fn Graph(
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
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M1.5 14H15v-1H2V0H1v13.5l.5.5zM3 11.5v-8l.5-.5h2l.5.5v8l-.5.5h-2l-.5-.5zm2-.5V4H4v7h1zm6-9.5v10l.5.5h2l.5-.5v-10l-.5-.5h-2l-.5.5zm2 .5v9h-1V2h1zm-6 9.5v-6l.5-.5h2l.5.5v6l-.5.5h-2l-.5-.5zm2-.5V6H8v5h1z"
        /> < title > { title } < / title > < / svg >
    }
}
