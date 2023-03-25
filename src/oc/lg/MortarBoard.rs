#[cfg(feature = "OcLgMortarBoard")]
use leptos::*;
#[cfg(feature = "OcLgMortarBoard")]
///This icon requires the feature `OcLgMortarBoard` to be enabled.
#[component]
pub fn MortarBoard(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M12.292 2.06v-.001l11.25 4.75a.749.749 0 0 1 0 1.382L19 10.108V15a.75.75 0 0 1-.11.391h-.001a2.84 2.84 0 0 1-.392.482c-.249.256-.625.58-1.163.896-1.08.638-2.776 1.23-5.334 1.23-.673 0-1.286-.041-1.846-.113a.75.75 0 0 1 .192-1.487c.492.063 1.042.1 1.654.1 2.317 0 3.746-.533 4.572-1.021.31-.178.596-.397.849-.65l.079-.085V10.74l-5.208 2.2a.75.75 0 0 1-.584 0L5.75 10.424v3.17c.502.129.96.391 1.327.758.579.578.923 1.41.923 2.428v4.5a.761.761 0 0 1-.345.634 2.157 2.157 0 0 1-.21.117 3.923 3.923 0 0 1-.52.213A6.121 6.121 0 0 1 5 22.532a6.092 6.092 0 0 1-1.925-.288 4.065 4.065 0 0 1-.52-.213 1.816 1.816 0 0 1-.22-.124.757.757 0 0 1-.335-.624v-4.5c0-1.02.344-1.85.923-2.43a2.904 2.904 0 0 1 1.327-.757V9.793L.458 8.19a.75.75 0 0 1 0-1.38l11.25-4.75a.75.75 0 0 1 .584 0ZM12 11.436 21.322 7.5 12 3.564 2.678 7.5ZM5 15c-.377 0-.745.141-1.017.413-.265.265-.483.7-.483 1.368v4.022c.299.105.797.228 1.5.228s1.201-.123 1.5-.228V16.78c0-.669-.218-1.103-.483-1.368A1.433 1.433 0 0 0 5 15Z"
        /> < title > { title } < / title > < / svg >
    }
}
