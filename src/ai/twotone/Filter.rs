#[cfg(feature = "AiTwotoneFilter")]
use leptos::*;
#[cfg(feature = "AiTwotoneFilter")]
///This icon requires the feature `AiTwotoneFilter` to be enabled.
#[component]
pub fn Filter(
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
        stroke_witdh = "0" style = style viewBox = "0 0 1024 1024" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" fill = "#D9D9D9" d =
        "M420.6 798h182.9V642H420.6zM411 561.4l9.5 16.6h183l9.5-16.6L811.3 226H212.7z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M880.1 154H143.9c-24.5 0-39.8 26.7-27.5 48L349 597.4V838c0 17.7 14.2 32 31.8 32h262.4c17.6 0 31.8-14.3 31.8-32V597.4L907.7 202c12.2-21.3-3.1-48-27.6-48zM603.5 798H420.6V642h182.9v156zm9.5-236.6l-9.5 16.6h-183l-9.5-16.6L212.7 226h598.6L613 561.4z"
        /> < title > { title } < / title > < / svg >
    }
}
