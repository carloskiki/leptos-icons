#[cfg(feature = "SiBoost")]
use leptos::*;
#[cfg(feature = "SiBoost")]
///This icon requires the feature `SiBoost` to be enabled.
#[component]
pub fn Boost(
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
        "M4.428 2.727l3.335 3.335c-.486.07-.903.276-1.32.624L.886 12.383c-1.181 1.18-1.181 3.194 0 4.375a2.41 2.41 0 0 0 1.598.834l17.088 3.681-3.335-3.333c.486-.07.903-.278 1.32-.626l5.557-5.695c1.181-1.181 1.181-3.196 0-4.377a2.411 2.411 0 0 0-1.598-.833zM11.653 6.2c.694 0 1.25.486 1.25 1.18 0 .695-.486 1.251-1.181 1.251-.695 0-1.25-.485-1.25-1.18s.555-1.251 1.18-1.251zm1.51 3.792c.049-.006.088.046.088.098-.139.694-.695 1.181-1.39 1.181-.694 0-1.32-.487-1.46-1.112 0 0 .002-.07.071 0 .487.278.972.348 1.32.278.346 0 .833-.07 1.32-.416a.092.092 0 0 1 .05-.029zm.723 2.511c.058.013.06.106.06.158-.209.903-.973 1.666-1.946 1.666a2.167 2.167 0 0 1-2.084-1.528c-.07-.07 0-.138.138-.138.695.347 1.39.416 1.877.416.486 0 1.18-.14 1.875-.556.035-.017.06-.022.08-.018zm.597 3.018c.049-.013.087.09.087.195-.278 1.181-1.25 2.085-2.5 2.155-1.251 0-2.293-.835-2.57-1.946 0-.139.068-.278.207-.209.834.486 1.737.556 2.362.556s1.529-.208 2.362-.694c.018-.035.036-.053.052-.057z"
        /> < title > { title } < / title > < / svg >
    }
}
