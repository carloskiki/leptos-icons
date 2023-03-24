#[cfg(feature = "ImAndroid")]
use leptos::*;
#[cfg(feature = "ImAndroid")]
///This icon requires the feature `ImAndroid` to be enabled.
#[component]
pub fn Android(
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
        "M14 6c-0.55 0-1 0.45-1 1v4c0 0.55 0.45 1 1 1s1-0.45 1-1v-4c0-0.55-0.45-1-1-1zM2 6c-0.55 0-1 0.45-1 1v4c0 0.55 0.45 1 1 1s1-0.45 1-1v-4c0-0.55-0.45-1-1-1zM3.5 11.5c0 0.828 0.672 1.5 1.5 1.5v0 2c0 0.55 0.45 1 1 1s1-0.45 1-1v-2h2v2c0 0.55 0.45 1 1 1s1-0.45 1-1v-2c0.828 0 1.5-0.672 1.5-1.5v-5.5h-9v5.5z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M12.472 5c-0.152-1.373-0.922-2.559-2.025-3.276l0.5-1.001c0.123-0.247 0.023-0.547-0.224-0.671s-0.547-0.023-0.671 0.224l-0.502 1.004-0.13-0.052c-0.446-0.148-0.924-0.229-1.42-0.229s-0.974 0.081-1.42 0.229l-0.13 0.052-0.502-1.004c-0.123-0.247-0.424-0.347-0.671-0.224s-0.347 0.424-0.224 0.671l0.5 1.001c-1.103 0.716-1.873 1.903-2.025 3.276v0.5h8.972v-0.5h-0.028zM6.5 4c-0.276 0-0.5-0.224-0.5-0.5s0.223-0.499 0.499-0.5c0 0 0.001 0 0.001 0s0.001-0 0.001-0c0.276 0.001 0.499 0.224 0.499 0.5s-0.224 0.5-0.5 0.5zM9.5 4c-0.276 0-0.5-0.224-0.5-0.5s0.223-0.499 0.499-0.5c0 0 0.001 0 0.001 0s0.001-0 0.002-0c0.276 0.001 0.499 0.224 0.499 0.5s-0.224 0.5-0.5 0.5z"
        /> < title > { title } < / title > < / svg >
    }
}
