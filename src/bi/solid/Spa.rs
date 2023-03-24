#[cfg(feature = "BiSolidSpa")]
use leptos::*;
#[cfg(feature = "BiSolidSpa")]
///This icon requires the feature `BiSolidSpa` to be enabled.
#[component]
pub fn Spa(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M12 16.114c-3.998-5.951-8.574-7.043-8.78-7.09L2 8.75V10c0 7.29 3.925 12 10 12 5.981 0 10-4.822 10-12V8.75l-1.22.274c-.206.047-4.782 1.139-8.78 7.09z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M11.274 3.767c-1.799 1.898-2.84 3.775-3.443 5.295 1.329.784 2.781 1.943 4.159 3.685 1.364-1.76 2.826-2.925 4.17-3.709-.605-1.515-1.646-3.383-3.435-5.271L12 3l-.726.767z"
        /> < title > { title } < / title > < / svg >
    }
}
