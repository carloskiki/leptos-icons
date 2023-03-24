#[cfg(feature = "ImSpades")]
use leptos::*;
#[cfg(feature = "ImSpades")]
///This icon requires the feature `ImSpades` to be enabled.
#[component]
pub fn Spades(
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
        "M12.775 5.44c-3.024-2.248-4.067-4.047-4.774-5.44v0c-0 0-0-0-0-0v0c-0.708 1.393-1.75 3.192-4.774 5.44-5.157 3.833-0.303 9.182 3.965 6.238-0.278 1.827-1.227 3.159-2.191 3.733v0.59h6v-0.59c-0.964-0.574-1.913-1.906-2.191-3.733 4.268 2.944 9.122-2.405 3.965-6.238z"
        /> < title > { title } < / title > < / svg >
    }
}
