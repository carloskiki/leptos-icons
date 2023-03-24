#[cfg(feature = "RiBusinessLineCloudOff")]
use leptos::*;
#[cfg(feature = "RiBusinessLineCloudOff")]
///This icon requires the feature `RiBusinessLineCloudOff` to be enabled.
#[component]
pub fn CloudOff(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M3.515 2.1l19.092 19.092-1.415 1.415-2.014-2.015A5.985 5.985 0 0 1 17 21H7A6 6 0 0 1 5.008 9.339a6.992 6.992 0 0 1 .353-2.563L2.1 3.514 3.515 2.1zM7 9c0 .081.002.163.006.243l.07 1.488-1.404.494A4.002 4.002 0 0 0 7 19h10c.186 0 .369-.013.548-.037L7.03 8.445C7.01 8.627 7 8.812 7 9zm5-7a7 7 0 0 1 6.992 7.339 6.003 6.003 0 0 1 3.212 8.65l-1.493-1.493a3.999 3.999 0 0 0-5.207-5.206L14.01 9.795C14.891 9.29 15.911 9 17 9a5 5 0 0 0-7.876-4.09l-1.43-1.43A6.97 6.97 0 0 1 12 2z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
