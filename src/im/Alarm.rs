#[cfg(feature = "ImAlarm")]
use leptos::*;
#[cfg(feature = "ImAlarm")]
///This icon requires the feature `ImAlarm` to be enabled.
#[component]
pub fn Alarm(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M8 2c-3.866 0-7 3.134-7 7s3.134 7 7 7 7-3.134 7-7-3.134-7-7-7zM8 14.625c-3.107 0-5.625-2.518-5.625-5.625s2.518-5.625 5.625-5.625c3.107 0 5.625 2.518 5.625 5.625s-2.518 5.625-5.625 5.625zM14.606 4.487c0.251-0.438 0.394-0.946 0.394-1.487 0-1.657-1.343-3-3-3-0.966 0-1.825 0.457-2.374 1.166 2.061 0.426 3.831 1.644 4.98 3.322v0zM6.374 1.166c-0.549-0.709-1.408-1.166-2.374-1.166-1.657 0-3 1.343-3 3 0 0.541 0.143 1.049 0.394 1.487 1.148-1.678 2.919-2.896 4.98-3.322z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d = "M8 9v-4h-1v5h4v-1z" /> <
        title > { title } < / title > < / svg >
    }
}
