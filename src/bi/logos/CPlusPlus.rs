#[cfg(feature = "BiLogosCPlusPlus")]
use leptos::*;
#[cfg(feature = "BiLogosCPlusPlus")]
///This icon requires the feature `BiLogosCPlusPlus` to be enabled.
#[component]
pub fn CPlusPlus(
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
        "M20.66 7a1.51 1.51 0 0 0-.55-.57l-7.34-4.24a1.67 1.67 0 0 0-1.54 0L3.89 6.43a1.68 1.68 0 0 0-.77 1.33v8.48a1.57 1.57 0 0 0 .22.76 1.51 1.51 0 0 0 .55.57l7.34 4.24a1.67 1.67 0 0 0 1.54 0l7.34-4.24a1.51 1.51 0 0 0 .55-.57 1.57 1.57 0 0 0 .22-.76V7.76a1.57 1.57 0 0 0-.22-.76zM12 17.92A5.92 5.92 0 1 1 17.13 9L16 9.71l-.36.2-1 .61A3 3 0 0 0 9 12a2.88 2.88 0 0 0 .4 1.48 3 3 0 0 0 5.13 0l2.6 1.52A5.94 5.94 0 0 1 12 17.92zm5.92-5.59h-.66V13h-.65v-.66H16v-.66h.66V11h.65v.66h.66zm2.47 0h-.66V13h-.66v-.66h-.65v-.66h.65V11h.66v.66h.66z"
        /> < title > { title } < / title > < / svg >
    }
}
