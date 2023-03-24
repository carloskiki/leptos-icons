#[cfg(feature = "BiLogosDrupal")]
use leptos::*;
#[cfg(feature = "BiLogosDrupal")]
///This icon requires the feature `BiLogosDrupal` to be enabled.
#[component]
pub fn Drupal(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M11.474 14.42a3.162 3.162 0 1 0 0 6.324 3.162 3.162 0 0 0 0-6.324zm3.936-.606a5.433 5.433 0 0 1 1.513 3.769 5.441 5.441 0 0 1-2.335 4.47c2.609-.803 4.771-2.767 5.737-5.142 1.338-3.288.09-5.761-1.999-8.005.066.288.103.592.103.898a4.175 4.175 0 0 1-3.019 4.01zm-3.577-4.003c0 1.34 1.087 2.419 2.42 2.419s2.423-1.087 2.423-2.419a2.418 2.418 0 0 0-2.417-2.417 2.425 2.425 0 0 0-2.426 2.417z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M7.303 21.093a5.435 5.435 0 0 1-1.276-3.51 5.441 5.441 0 0 1 4.8-5.408 4.162 4.162 0 0 1 3.99-6.492c-1.361-1.176-2.724-2.369-3.799-3.672.547 5.714-5.2 3.638-7.332 8.904-1.422 3.527-.138 7.892 3.617 10.178z"
        /> < title > { title } < / title > < / svg >
    }
}
