#[cfg(feature = "AiOutlinedAccountBook")]
use leptos::*;
#[cfg(feature = "AiOutlinedAccountBook")]
///This icon requires the feature `AiOutlinedAccountBook` to be enabled.
#[component]
pub fn AccountBook(
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
        stroke_witdh = "0" style = style class = "icon" viewBox = "0 0 1024 1024" width =
        size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M880 184H712v-64c0-4.4-3.6-8-8-8h-56c-4.4 0-8 3.6-8 8v64H384v-64c0-4.4-3.6-8-8-8h-56c-4.4 0-8 3.6-8 8v64H144c-17.7 0-32 14.3-32 32v664c0 17.7 14.3 32 32 32h736c17.7 0 32-14.3 32-32V216c0-17.7-14.3-32-32-32zm-40 656H184V256h128v48c0 4.4 3.6 8 8 8h56c4.4 0 8-3.6 8-8v-48h256v48c0 4.4 3.6 8 8 8h56c4.4 0 8-3.6 8-8v-48h128v584zM639.5 414h-45c-3 0-5.8 1.7-7.1 4.4L514 563.8h-2.8l-73.4-145.4a8 8 0 0 0-7.1-4.4h-46c-1.3 0-2.7.3-3.8 1-3.9 2.1-5.3 7-3.2 10.9l89.3 164h-48.6c-4.4 0-8 3.6-8 8v21.3c0 4.4 3.6 8 8 8h65.1v33.7h-65.1c-4.4 0-8 3.6-8 8v21.3c0 4.4 3.6 8 8 8h65.1V752c0 4.4 3.6 8 8 8h41.3c4.4 0 8-3.6 8-8v-53.8h65.4c4.4 0 8-3.6 8-8v-21.3c0-4.4-3.6-8-8-8h-65.4v-33.7h65.4c4.4 0 8-3.6 8-8v-21.3c0-4.4-3.6-8-8-8h-49.1l89.3-164.1c.6-1.2 1-2.5 1-3.8.1-4.4-3.4-8-7.9-8z"
        /> < title > { title } < / title > < / svg >
    }
}
