#[cfg(feature = "TiTrash")]
use leptos::*;
#[cfg(feature = "TiTrash")]
///This icon requires the feature `TiTrash` to be enabled.
#[component]
pub fn Trash(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M18 7h-1v-1c0-1.104-.896-2-2-2h-7c-1.104 0-2 .896-2 2v1h-1c-.552 0-1 .448-1 1s.448 1 1 1v8c0 2.206 1.794 4 4 4h5c2.206 0 4-1.794 4-4v-8c.552 0 1-.448 1-1s-.448-1-1-1zm-10-1h7v1h-7v-1zm8 11c0 1.104-.896 2-2 2h-5c-1.104 0-2-.896-2-2v-8h9v8zM8.5 10.5c-.275 0-.5.225-.5.5v6c0 .275.225.5.5.5s.5-.225.5-.5v-6c0-.275-.225-.5-.5-.5zM10.5 10.5c-.275 0-.5.225-.5.5v6c0 .275.225.5.5.5s.5-.225.5-.5v-6c0-.275-.225-.5-.5-.5zM12.5 10.5c-.275 0-.5.225-.5.5v6c0 .275.225.5.5.5s.5-.225.5-.5v-6c0-.275-.225-.5-.5-.5zM14.5 10.5c-.275 0-.5.225-.5.5v6c0 .275.225.5.5.5s.5-.225.5-.5v-6c0-.275-.225-.5-.5-.5z"
        /> < title > { title } < / title > < / svg >
    }
}
