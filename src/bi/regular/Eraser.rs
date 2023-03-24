#[cfg(feature = "BiRegularEraser")]
use leptos::*;
#[cfg(feature = "BiRegularEraser")]
///This icon requires the feature `BiRegularEraser` to be enabled.
#[component]
pub fn Eraser(
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
        "m2.586 15.408 4.299 4.299a.996.996 0 0 0 .707.293h12.001v-2h-6.958l7.222-7.222c.78-.779.78-2.049 0-2.828L14.906 3a2.003 2.003 0 0 0-2.828 0l-4.75 4.749-4.754 4.843a2.007 2.007 0 0 0 .012 2.816zM13.492 4.414l4.95 4.95-2.586 2.586L10.906 7l2.586-2.586zM8.749 9.156l.743-.742 4.95 4.95-4.557 4.557a1.026 1.026 0 0 0-.069.079h-1.81l-4.005-4.007 4.748-4.837z"
        /> < title > { title } < / title > < / svg >
    }
}
