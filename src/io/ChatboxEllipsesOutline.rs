#[cfg(feature = "IoChatboxEllipsesOutline")]
use leptos::*;
#[cfg(feature = "IoChatboxEllipsesOutline")]
///This icon requires the feature `IoChatboxEllipsesOutline` to be enabled.
#[component]
pub fn ChatboxEllipsesOutline(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M408,64H104a56.16,56.16,0,0,0-56,56V312a56.16,56.16,0,0,0,56,56h40v80l93.72-78.14a8,8,0,0,1,5.13-1.86H408a56.16,56.16,0,0,0,56-56V120A56.16,56.16,0,0,0,408,64Z"
        style = "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" /><
        circle xmlns = "http://www.w3.org/2000/svg" cx = "160" cy = "216" r = "32" /><
        circle xmlns = "http://www.w3.org/2000/svg" cx = "256" cy = "216" r = "32" /><
        circle xmlns = "http://www.w3.org/2000/svg" cx = "352" cy = "216" r = "32" /> <
        title > { title } < / title > < / svg >
    }
}
