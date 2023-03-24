#[cfg(feature = "HiMdSolidPauseCircle")]
use leptos::*;
#[cfg(feature = "HiMdSolidPauseCircle")]
///This icon requires the feature `HiMdSolidPauseCircle` to be enabled.
#[component]
pub fn PauseCircle(
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M2 10C2 5.58172 5.58172 2 10 2C14.4183 2 18 5.58172 18 10C18 14.4183 14.4183 18 10 18C5.58172 18 2 14.4183 2 10ZM7 7.75C7 7.33579 7.33579 7 7.75 7H8.25C8.66421 7 9 7.33579 9 7.75V12.25C9 12.6642 8.66421 13 8.25 13H7.75C7.33579 13 7 12.6642 7 12.25V7.75ZM11 7.75C11 7.33579 11.3358 7 11.75 7H12.25C12.6642 7 13 7.33579 13 7.75V12.25C13 12.6642 12.6642 13 12.25 13H11.75C11.3358 13 11 12.6642 11 12.25V7.75Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
