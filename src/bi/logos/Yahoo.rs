#[cfg(feature = "BiLogosYahoo")]
use leptos::*;
#[cfg(feature = "BiLogosYahoo")]
///This icon requires the feature `BiLogosYahoo` to be enabled.
#[component]
pub fn Yahoo(
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
        "M13.131 21s-.63-.114-1.138-.114c-.457 0-1.142.114-1.142.114l.143-7.646C9.933 11.52 6.814 5.933 4.868 3c.979.223 1.391.209 2.374 0l.015.025c1.239 2.194 3.135 5.254 4.736 7.905C13.575 8.325 16.064 4.258 16.74 3c.765.201 1.536.193 2.392 0-.9 1.213-4.175 6.88-6.153 10.354L13.125 21h.006z"
        /> < title > { title } < / title > < / svg >
    }
}
