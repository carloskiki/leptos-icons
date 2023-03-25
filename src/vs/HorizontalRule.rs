#[cfg(feature = "VsHorizontalRule")]
use leptos::*;
#[cfg(feature = "VsHorizontalRule")]
///This icon requires the feature `VsHorizontalRule` to be enabled.
#[component]
pub fn HorizontalRule(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M6.432 10h.823V4h-.823v2.61h-2.61V4H3v6h.823V7.394h2.61V10zm5.668 0h.9l-1.28-2.63c.131-.058.26-.134.389-.23a1.666 1.666 0 0 0 .585-.797c.064-.171.096-.364.096-.58a1.77 1.77 0 0 0-.082-.557 1.644 1.644 0 0 0-.22-.446 1.504 1.504 0 0 0-.31-.341 1.864 1.864 0 0 0-.737-.373A1.446 1.446 0 0 0 11.1 4H8.64v6h.824V7.518h1.467L12.1 10zm-.681-3.32a.874.874 0 0 1-.293.055H9.463V4.787h1.663a.87.87 0 0 1 .576.24.956.956 0 0 1 .306.737c0 .168-.029.314-.087.437a.91.91 0 0 1-.503.479zM13 12H3v1h10v-1z"
        /> < title > { title } < / title > < / svg >
    }
}
