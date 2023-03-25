#[cfg(feature = "BiRegularPaint")]
use leptos::*;
#[cfg(feature = "BiRegularPaint")]
///This icon requires the feature `BiRegularPaint` to be enabled.
#[component]
pub fn Paint(
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
        "M7.061 22c1.523 0 2.84-.543 3.91-1.613 1.123-1.123 1.707-2.854 1.551-4.494l8.564-8.564a3.123 3.123 0 0 0-.002-4.414c-1.178-1.18-3.234-1.18-4.412 0l-8.884 8.884c-1.913.169-3.807 1.521-3.807 3.919 0 .303.021.588.042.86.08 1.031.109 1.418-1.471 2.208a1.001 1.001 0 0 0-.122 1.717C2.52 20.563 4.623 22 7.061 22c-.001 0-.001 0 0 0zM18.086 4.328a1.144 1.144 0 0 1 1.586.002 1.12 1.12 0 0 1 0 1.584L12 13.586 10.414 12l7.672-7.672zM6.018 16.423c-.018-.224-.037-.458-.037-.706 0-1.545 1.445-1.953 2.21-1.953.356 0 .699.073.964.206.945.475 1.26 1.293 1.357 1.896.177 1.09-.217 2.368-.956 3.107C8.865 19.664 8.049 20 7.061 20H7.06c-.75 0-1.479-.196-2.074-.427 1.082-.973 1.121-1.989 1.032-3.15z"
        /> < title > { title } < / title > < / svg >
    }
}
