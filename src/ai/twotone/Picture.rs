#[cfg(feature = "AiTwotonePicture")]
use leptos::*;
#[cfg(feature = "AiTwotonePicture")]
///This icon requires the feature `AiTwotonePicture` to be enabled.
#[component]
pub fn Picture(
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
        stroke_witdh = "0" style = style viewBox = "0 0 1024 1024" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" fill = "#333" d =
        "M928 160H96c-17.7 0-32 14.3-32 32v640c0 17.7 14.3 32 32 32h832c17.7 0 32-14.3 32-32V192c0-17.7-14.3-32-32-32zm-40 632H136v-39.9l138.5-164.3 150.1 178L658.1 489 888 761.6V792zm0-129.8L664.2 396.8c-3.2-3.8-9-3.8-12.2 0L424.6 666.4l-144-170.7c-3.2-3.8-9-3.8-12.2 0L136 652.7V232h752v430.2z"
        />< path xmlns = "http://www.w3.org/2000/svg" fill = "#E6E6E6" d =
        "M424.6 765.8l-150.1-178L136 752.1V792h752v-30.4L658.1 489z" />< path xmlns =
        "http://www.w3.org/2000/svg" fill = "#E6E6E6" d =
        "M136 652.7l132.4-157c3.2-3.8 9-3.8 12.2 0l144 170.7L652 396.8c3.2-3.8 9-3.8 12.2 0L888 662.2V232H136v420.7zM304 280a88 88 0 1 1 0 176 88 88 0 0 1 0-176z"
        />< path xmlns = "http://www.w3.org/2000/svg" fill = "#E6E6E6" d =
        "M276 368a28 28 0 1 0 56 0 28 28 0 1 0-56 0z" />< path xmlns =
        "http://www.w3.org/2000/svg" fill = "#333" d =
        "M304 456a88 88 0 1 0 0-176 88 88 0 0 0 0 176zm0-116c15.5 0 28 12.5 28 28s-12.5 28-28 28-28-12.5-28-28 12.5-28 28-28z"
        /> < title > { title } < / title > < / svg >
    }
}
