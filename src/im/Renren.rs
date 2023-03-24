#[cfg(feature = "ImRenren")]
use leptos::*;
#[cfg(feature = "ImRenren")]
///This icon requires the feature `ImRenren` to be enabled.
#[component]
pub fn Renren(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" xmlns : xlink = "http://www.w3.org/1999/xlink" fill
        = "#000000" d =
        "M6.644 0.166c-3.769 0.634-6.644 3.913-6.644 7.862 0 1.963 0.713 3.759 1.887 5.15 2.791-1.35 4.744-4.406 4.756-7.966v-5.047z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M9.356 0.166c3.769 0.634 6.644 3.913 6.644 7.862 0 1.963-0.713 3.759-1.887 5.15-2.791-1.35-4.744-4.406-4.756-7.966v-5.047z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M7.972 10.041c-0.497 2.056-1.981 3.813-3.828 4.981 1.138 0.622 2.441 0.978 3.828 0.978s2.691-0.356 3.828-0.978c-1.847-1.169-3.331-2.925-3.828-4.981z"
        /> < title > { title } < / title > < / svg >
    }
}
