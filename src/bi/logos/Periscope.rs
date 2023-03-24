#[cfg(feature = "BiLogosPeriscope")]
use leptos::*;
#[cfg(feature = "BiLogosPeriscope")]
///This icon requires the feature `BiLogosPeriscope` to be enabled.
#[component]
pub fn Periscope(
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
        "M12.102 21c1.406 0 6.985-6.329 6.985-10.571C19.087 6.368 15.915 3 12.102 3c-4.017 0-7.188 3.366-7.188 7.429C4.913 14.67 10.492 21 12.102 21zM10.979 5.885a4.696 4.696 0 0 1 1.143-.139c2.25 0 4.186 1.913 4.186 4.398 0 2.238-1.936 4.151-4.196 4.151-2.509 0-4.444-1.913-4.444-4.151 0-1.047.338-1.98.922-2.723v.022c0 .934.755 1.676 1.688 1.676.933.002 1.722-.764 1.722-1.697a1.68 1.68 0 0 0-1.02-1.54l-.001.003z"
        /> < title > { title } < / title > < / svg >
    }
}
