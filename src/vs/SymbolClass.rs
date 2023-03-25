#[cfg(feature = "VsSymbolClass")]
use leptos::*;
#[cfg(feature = "VsSymbolClass")]
///This icon requires the feature `VsSymbolClass` to be enabled.
#[component]
pub fn SymbolClass(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M11.34 9.71h.71l2.67-2.67v-.71L13.38 5h-.7l-1.82 1.81h-5V5.56l1.86-1.85V3l-2-2H5L1 5v.71l2 2h.71l1.14-1.15v5.79l.5.5H10v.52l1.33 1.34h.71l2.67-2.67v-.71L13.37 10h-.7l-1.86 1.85h-5v-4H10v.48l1.34 1.38zm1.69-3.65l.63.63-2 2-.63-.63 2-2zm0 5l.63.63-2 2-.63-.63 2-2zM3.35 6.65l-1.29-1.3 3.29-3.29 1.3 1.29-3.3 3.3z"
        /> < title > { title } < / title > < / svg >
    }
}
