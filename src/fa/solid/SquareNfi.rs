#[cfg(feature = "FaSolidSquareNfi")]
use leptos::*;
#[cfg(feature = "FaSolidSquareNfi")]
///This icon requires the feature `FaSolidSquareNfi` to be enabled.
#[component]
pub fn SquareNfi(
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
        stroke_witdh = "0" style = style viewBox = "0 0 448 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M0 96C0 60.7 28.7 32 64 32H384c35.3 0 64 28.7 64 64V416c0 35.3-28.7 64-64 64H64c-35.3 0-64-28.7-64-64V96zm75.7 64.6C68.8 162.5 64 168.8 64 176V336c0 8.8 7.2 16 16 16s16-7.2 16-16V233.8l66.3 110.5c3.7 6.2 11.1 9.1 18 7.2s11.7-8.2 11.7-15.4V176c0-8.8-7.2-16-16-16s-16 7.2-16 16V278.2L93.7 167.8c-3.7-6.2-11.1-9.1-18-7.2zM224 176v64 96c0 8.8 7.2 16 16 16s16-7.2 16-16V256h48c8.8 0 16-7.2 16-16s-7.2-16-16-16H256V192h48c8.8 0 16-7.2 16-16s-7.2-16-16-16H240c-8.8 0-16 7.2-16 16zm160 0c0-8.8-7.2-16-16-16s-16 7.2-16 16V336c0 8.8 7.2 16 16 16s16-7.2 16-16V176z"
        /> < title > { title } < / title > < / svg >
    }
}
