#[cfg(feature = "RiCommunicationFillChatQuote")]
use leptos::*;
#[cfg(feature = "RiCommunicationFillChatQuote")]
///This icon requires the feature `RiCommunicationFillChatQuote` to be enabled.
#[component]
pub fn ChatQuote(
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
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0H24V24H0z" />< path d
        =
        "M21 3c.552 0 1 .448 1 1v14c0 .552-.448 1-1 1H6.455L2 22.5V4c0-.552.448-1 1-1h18zM10.962 8.1l-.447-.688C8.728 8.187 7.5 9.755 7.5 11.505c0 .995.277 1.609.792 2.156.324.344.837.589 1.374.589.966 0 1.75-.784 1.75-1.75 0-.92-.711-1.661-1.614-1.745-.16-.015-.324-.012-.479.01v-.092c.006-.422.092-1.633 1.454-2.466l.185-.107-.447-.688zm4.553-.688c-1.787.775-3.015 2.343-3.015 4.093 0 .995.277 1.609.792 2.156.324.344.837.589 1.374.589.966 0 1.75-.784 1.75-1.75 0-.92-.711-1.661-1.614-1.745-.16-.015-.324-.012-.479.01 0-.313-.029-1.762 1.639-2.665z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
