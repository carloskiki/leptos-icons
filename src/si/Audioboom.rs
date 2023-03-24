#[cfg(feature = "SiAudioboom")]
use leptos::*;
#[cfg(feature = "SiAudioboom")]
///This icon requires the feature `SiAudioboom` to be enabled.
#[component]
pub fn Audioboom(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M12 24C5.373 24 0 18.627 0 12S5.373 0 12 0s12 5.373 12 12-5.373 12-12 12zM7.425 3.214c-.621 0-1.125.503-1.125 1.124v6a1.124 1.124 0 0 0 2.25 0v-6c0-.62-.504-1.124-1.125-1.124zm0 9.314c-.621 0-1.125.503-1.125 1.125v6a1.124 1.124 0 0 0 2.25 0v-6c0-.622-.504-1.125-1.125-1.125zm4.152-6.856c-.621 0-1.125.504-1.125 1.125v10.388a1.124 1.124 0 0 0 2.25 0V6.797c0-.621-.504-1.125-1.125-1.125zm4.151 6.856c-.62 0-1.124.503-1.124 1.125v1.056a1.124 1.124 0 1 0 2.249 0v-1.056c0-.622-.504-1.125-1.125-1.125zm0-4.37c-.62 0-1.124.503-1.124 1.124v1.056a1.124 1.124 0 0 0 2.249 0V9.282c0-.62-.504-1.124-1.125-1.124zm4.152 2.422c-.62 0-1.124.503-1.124 1.124v.574a1.124 1.124 0 1 0 2.249 0v-.574c0-.62-.504-1.124-1.125-1.124Z"
        /> < title > { title } < / title > < / svg >
    }
}
