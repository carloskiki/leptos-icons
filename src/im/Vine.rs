#[cfg(feature = "ImVine")]
use leptos::*;
#[cfg(feature = "ImVine")]
///This icon requires the feature `ImVine` to be enabled.
#[component]
pub fn Vine(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" xmlns : xlink = "http://www.w3.org/1999/xlink" fill
        = "#000000" d =
        "M15.012 7.953c-0.412 0.094-0.809 0.137-1.169 0.137-2.019 0-3.572-1.409-3.572-3.862 0-1.203 0.466-1.825 1.122-1.825 0.625 0 1.041 0.559 1.041 1.697 0 0.647-0.172 1.356-0.3 1.775 0 0 0.622 1.084 2.322 0.753 0.363-0.803 0.556-1.841 0.556-2.75 0-2.45-1.25-3.878-3.541-3.878-2.356 0-3.734 1.809-3.734 4.197 0 2.366 1.106 4.394 2.928 5.319-0.766 1.534-1.741 2.884-2.759 3.903-1.844-2.231-3.513-5.206-4.197-11.016h-2.722c1.259 9.675 5.006 12.756 6 13.347 0.559 0.337 1.044 0.322 1.556 0.031 0.806-0.456 3.222-2.875 4.563-5.703 0.563 0 1.238-0.066 1.909-0.219v-1.906z"
        /> < title > { title } < / title > < / svg >
    }
}
