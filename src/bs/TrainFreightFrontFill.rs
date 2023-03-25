#[cfg(feature = "BsTrainFreightFrontFill")]
use leptos::*;
#[cfg(feature = "BsTrainFreightFrontFill")]
///This icon requires the feature `BsTrainFreightFrontFill` to be enabled.
#[component]
pub fn TrainFreightFrontFill(
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
        stroke_witdh = "0" style = style width = "16" height = "16" fill = "currentColor"
        class = "bi bi-train-freight-front-fill" viewBox = "0 0 16 16" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M5.736 0a1.5 1.5 0 0 0-.67.158L1.828 1.776A1.5 1.5 0 0 0 1 3.118v5.51l2-.6V5a1 1 0 0 1 1-1h8a1 1 0 0 1 1 1v3.028l2 .6v-5.51a1.5 1.5 0 0 0-.83-1.342L10.936.158A1.5 1.5 0 0 0 10.264 0H5.736ZM15 9.672l-5.503-1.65A.5.5 0 0 0 9.353 8H8.5v8h4a2.5 2.5 0 0 0 2.5-2.5V9.672ZM7.5 16V8h-.853a.5.5 0 0 0-.144.021L1 9.672V13.5A2.5 2.5 0 0 0 3.5 16h4Zm-1-14h3a.5.5 0 0 1 0 1h-3a.5.5 0 0 1 0-1ZM12 5v2.728l-2.216-.665A1.5 1.5 0 0 0 9.354 7H8.5V5H12ZM7.5 5v2h-.853a1.5 1.5 0 0 0-.431.063L4 7.728V5h3.5Zm-4 5a.5.5 0 1 1 0 1 .5.5 0 0 1 0-1Zm9 0a.5.5 0 1 1 0 1 .5.5 0 0 1 0-1ZM5 13a1 1 0 1 1-2 0 1 1 0 0 1 2 0Zm7 1a1 1 0 1 1 0-2 1 1 0 0 1 0 2Z"
        /> < title > { title } < / title > < / svg >
    }
}
