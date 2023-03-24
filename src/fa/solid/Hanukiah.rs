#[cfg(feature = "FaSolidHanukiah")]
use leptos::*;
#[cfg(feature = "FaSolidHanukiah")]
///This icon requires the feature `FaSolidHanukiah` to be enabled.
#[component]
pub fn Hanukiah(
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
        stroke_witdh = "0" style = style viewBox = "0 0 640 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M314.2 3.3C309.1 12.1 296 36.6 296 56c0 13.3 10.7 24 24 24s24-10.7 24-24c0-19.4-13.1-43.9-18.2-52.7C324.6 1.2 322.4 0 320 0s-4.6 1.2-5.8 3.3zm-288 48C21.1 60.1 8 84.6 8 104c0 13.3 10.7 24 24 24s24-10.7 24-24c0-19.4-13.1-43.9-18.2-52.7C36.6 49.2 34.4 48 32 48s-4.6 1.2-5.8 3.3zM88 104c0 13.3 10.7 24 24 24s24-10.7 24-24c0-19.4-13.1-43.9-18.2-52.7c-1.2-2.1-3.4-3.3-5.8-3.3s-4.6 1.2-5.8 3.3C101.1 60.1 88 84.6 88 104zm82.2-52.7C165.1 60.1 152 84.6 152 104c0 13.3 10.7 24 24 24s24-10.7 24-24c0-19.4-13.1-43.9-18.2-52.7c-1.2-2.1-3.4-3.3-5.8-3.3s-4.6 1.2-5.8 3.3zM216 104c0 13.3 10.7 24 24 24s24-10.7 24-24c0-19.4-13.1-43.9-18.2-52.7c-1.2-2.1-3.4-3.3-5.8-3.3s-4.6 1.2-5.8 3.3C229.1 60.1 216 84.6 216 104zM394.2 51.3C389.1 60.1 376 84.6 376 104c0 13.3 10.7 24 24 24s24-10.7 24-24c0-19.4-13.1-43.9-18.2-52.7c-1.2-2.1-3.4-3.3-5.8-3.3s-4.6 1.2-5.8 3.3zM440 104c0 13.3 10.7 24 24 24s24-10.7 24-24c0-19.4-13.1-43.9-18.2-52.7c-1.2-2.1-3.4-3.3-5.8-3.3s-4.6 1.2-5.8 3.3C453.1 60.1 440 84.6 440 104zm82.2-52.7C517.1 60.1 504 84.6 504 104c0 13.3 10.7 24 24 24s24-10.7 24-24c0-19.4-13.1-43.9-18.2-52.7c-1.2-2.1-3.4-3.3-5.8-3.3s-4.6 1.2-5.8 3.3zM584 104c0 13.3 10.7 24 24 24s24-10.7 24-24c0-19.4-13.1-43.9-18.2-52.7c-1.2-2.1-3.4-3.3-5.8-3.3s-4.6 1.2-5.8 3.3C597.1 60.1 584 84.6 584 104zM112 160c-8.8 0-16 7.2-16 16v96 16h32V272 176c0-8.8-7.2-16-16-16zm64 0c-8.8 0-16 7.2-16 16v96 16h32V272 176c0-8.8-7.2-16-16-16zm64 0c-8.8 0-16 7.2-16 16v96 16h32V272 176c0-8.8-7.2-16-16-16zm160 0c-8.8 0-16 7.2-16 16v96 16h32V272 176c0-8.8-7.2-16-16-16zm64 0c-8.8 0-16 7.2-16 16v96 16h32V272 176c0-8.8-7.2-16-16-16zm64 0c-8.8 0-16 7.2-16 16v96 16h32V272 176c0-8.8-7.2-16-16-16zM352 144c0-17.7-14.3-32-32-32s-32 14.3-32 32V320H96c-17.7 0-32-14.3-32-32V192c0-17.7-14.3-32-32-32s-32 14.3-32 32v96c0 53 43 96 96 96H288v64H160c-17.7 0-32 14.3-32 32s14.3 32 32 32H320 480c17.7 0 32-14.3 32-32s-14.3-32-32-32H352V384H544c53 0 96-43 96-96V192c0-17.7-14.3-32-32-32s-32 14.3-32 32v96c0 17.7-14.3 32-32 32H352V144z"
        /> < title > { title } < / title > < / svg >
    }
}
