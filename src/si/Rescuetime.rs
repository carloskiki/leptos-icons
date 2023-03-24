#[cfg(feature = "SiRescuetime")]
use leptos::*;
#[cfg(feature = "SiRescuetime")]
///This icon requires the feature `SiRescuetime` to be enabled.
#[component]
pub fn Rescuetime(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "m24 7.626v8.749c0 .597-.485 1.092-1.091 1.092h-5.447v5.452c0 .596-.485 1.092-1.091 1.092h-8.742c-.596 0-1.091-.486-1.091-1.092v-5.452h-5.447c-.596 0-1.091-.485-1.091-1.092v-8.749c0-.597.485-1.092 1.091-1.092h5.447v-5.452c0-.596.485-1.092 1.091-1.092h8.742c.596 0 1.091.485 1.091 1.092v5.452h5.447c.596 0 1.091.495 1.091 1.092zm-3.325 4.339-2.192-1.649.333 1.042-4.891-.344c.152.304.243.638.243.992 0 .343-.081.667-.213.95l4.871-.364-.323 1.022zm-7.579.03-.495-8 1.021.324-1.647-2.185-1.647 2.195 1.04-.334-.454 8c0 .597.485 1.093 1.091 1.093.596 0 1.091-.486 1.091-1.093z"
        /> < title > { title } < / title > < / svg >
    }
}
