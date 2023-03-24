#[cfg(feature = "SiRollupdotjs")]
use leptos::*;
#[cfg(feature = "SiRollupdotjs")]
///This icon requires the feature `SiRollupdotjs` to be enabled.
#[component]
pub fn Rollupdotjs(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M3.42.0002a.37.37 0 00-.369.37V19.885c.577-1.488 1.557-3.6168 3.1378-6.5297C11.8885 2.876 12.6355 1.8191 15.6043 1.8191c1.56 0 3.1338.704 4.1518 1.9549A7.9616 7.9616 0 0013.1014.0002zM16.1393 2.544c-1.19.01-2.257.466-2.6979 1.498-.967 2.2558 1.624 4.7667 2.7569 4.5677 1.4419-.255-.255-3.5628-.255-3.5628 2.2049 4.1558 1.6969 2.8838-2.2899 6.6996C9.6666 15.5623 5.596 23.6188 5.002 23.9568a.477.477 0 01-.08.043H20.558a.373.373 0 00.33-.538l-4.0878-8.0915a.37.37 0 01.144-.488 7.9596 7.9596 0 004.0048-6.9126c0-1.4249-.373-2.7608-1.03-3.9198-.9269-.9519-2.4298-1.5159-3.7787-1.5059z"
        /> < title > { title } < / title > < / svg >
    }
}
