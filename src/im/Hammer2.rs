#[cfg(feature = "ImHammer2")]
use leptos::*;
#[cfg(feature = "ImHammer2")]
///This icon requires the feature `ImHammer2` to be enabled.
#[component]
pub fn Hammer2(
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
        "M15.784 14.309l-8.572-7.804 0.399-0.4c0.326-0.327 0.503-0.75 0.53-1.181 0.016-0.007 0.031-0.014 0.046-0.023l1.609-1.006c0.218-0.256 0.202-0.66-0.036-0.898l-2.799-2.806c-0.237-0.238-0.641-0.254-0.896-0.036l-1.004 1.614c-0.008 0.015-0.015 0.031-0.022 0.046-0.43 0.027-0.852 0.204-1.178 0.531l-1.522 1.527c-0.327 0.327-0.503 0.75-0.53 1.181-0.016 0.007-0.031 0.014-0.046 0.023l-1.609 1.006c-0.218 0.256-0.202 0.66 0.036 0.898l2.799 2.806c0.237 0.238 0.641 0.254 0.896 0.036l1.004-1.614c0.008-0.015 0.015-0.031 0.023-0.046 0.43-0.027 0.852-0.204 1.178-0.531l0.442-0.443 7.783 8.596c0.226 0.249 0.573 0.289 0.773 0.089l0.787-0.789c0.199-0.2 0.159-0.549-0.089-0.775z"
        /> < title > { title } < / title > < / svg >
    }
}
