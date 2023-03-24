#[cfg(feature = "BiRegularPen")]
use leptos::*;
#[cfg(feature = "BiRegularPen")]
///This icon requires the feature `BiRegularPen` to be enabled.
#[component]
pub fn Pen(
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
        "M18.404 2.998c-.757-.754-2.077-.751-2.828.005l-1.784 1.791L11.586 7H7a.998.998 0 0 0-.939.658l-4 11c-.133.365-.042.774.232 1.049l2 2a.997.997 0 0 0 1.049.232l11-4A.998.998 0 0 0 17 17v-4.586l2.207-2.207v-.001h.001L21 8.409c.378-.378.586-.881.585-1.415 0-.535-.209-1.038-.588-1.415l-2.593-2.581zm-3.111 8.295A.996.996 0 0 0 15 12v4.3l-9.249 3.363 4.671-4.671c.026.001.052.008.078.008A1.5 1.5 0 1 0 9 13.5c0 .026.007.052.008.078l-4.671 4.671L7.7 9H12c.266 0 .52-.105.707-.293L14.5 6.914 17.086 9.5l-1.793 1.793zm3.206-3.208-2.586-2.586 1.079-1.084 2.593 2.581-1.086 1.089z"
        /> < title > { title } < / title > < / svg >
    }
}
