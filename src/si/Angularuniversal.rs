#[cfg(feature = "SiAngularuniversal")]
use leptos::*;
#[cfg(feature = "SiAngularuniversal")]
///This icon requires the feature `SiAngularuniversal` to be enabled.
#[component]
pub fn Angularuniversal(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M15.6 11.28v1.44a.48.48 0 0 1-.48.48H8.88a.48.48 0 0 1-.48-.48v-1.44a.48.48 0 0 1 .48-.48h6.24a.48.48 0 0 1 .48.48zM12 15.6a1.2 1.2 0 1 0 0 2.4 1.2 1.2 0 0 0 0-2.4zm3.12-8.4H8.88a.48.48 0 0 0-.48.48v1.44c0 .265.215.48.48.48h6.24a.48.48 0 0 0 .48-.48V7.68a.48.48 0 0 0-.48-.48zm8.04-3.204l-1.716 14.736L11.976 24 2.52 18.732.84 3.996 11.976 0 23.16 3.996zM16.8 6.24a1.44 1.44 0 0 0-1.44-1.44H8.64A1.44 1.44 0 0 0 7.2 6.24v11.52c0 .795.645 1.44 1.44 1.44h6.72a1.44 1.44 0 0 0 1.44-1.44V6.24z"
        /> < title > { title } < / title > < / svg >
    }
}
