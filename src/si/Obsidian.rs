#[cfg(feature = "SiObsidian")]
use leptos::*;
#[cfg(feature = "SiObsidian")]
///This icon requires the feature `SiObsidian` to be enabled.
#[component]
pub fn Obsidian(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M15.074 0C12.586 1.374 10.1 2.749 7.613 4.124l.823 4.266 6.365-5.015zm.172.059l-.269 3.314 4.497 2.752zm-.353 3.466L8.487 8.576l7.39 15.367 1.177-2.359L19.58 6.4c-.012-.009-4.688-2.875-4.688-2.875zm-7.425.779l-3.05 6.594L9.033 21.51l-.74-12.934-.012-.064zm1.025 4.688l.73 12.784L15.71 24Z"
        /> < title > { title } < / title > < / svg >
    }
}
