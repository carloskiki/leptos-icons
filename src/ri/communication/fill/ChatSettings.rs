#[cfg(feature = "RiCommunicationFillChatSettings")]
use leptos::*;
#[cfg(feature = "RiCommunicationFillChatSettings")]
///This icon requires the feature `RiCommunicationFillChatSettings` to be enabled.
#[component]
pub fn ChatSettings(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M6.455 19L2 22.5V4a1 1 0 0 1 1-1h18a1 1 0 0 1 1 1v14a1 1 0 0 1-1 1H6.455zm1.69-6.929l-.975.563 1 1.732.976-.563c.501.51 1.14.887 1.854 1.071V16h2v-1.126a3.996 3.996 0 0 0 1.854-1.071l.976.563 1-1.732-.975-.563a4.004 4.004 0 0 0 0-2.142l.975-.563-1-1.732-.976.563A3.996 3.996 0 0 0 13 7.126V6h-2v1.126a3.996 3.996 0 0 0-1.854 1.071l-.976-.563-1 1.732.975.563a4.004 4.004 0 0 0 0 2.142zM12 13a2 2 0 1 1 0-4 2 2 0 0 1 0 4z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
