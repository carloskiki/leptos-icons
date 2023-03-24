#[cfg(feature = "SiGoogletagmanager")]
use leptos::*;
#[cfg(feature = "SiGoogletagmanager")]
///This icon requires the feature `SiGoogletagmanager` to be enabled.
#[component]
pub fn Googletagmanager(
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
        "M12.003 0a3 3 0 0 0-2.121 5.121l6.865 6.865-4.446 4.541 1.745 1.836a3.432 3.432 0 0 1 .7.739l.012.011-.001.002a3.432 3.432 0 0 1 .609 1.953 3.432 3.432 0 0 1-.09.78l7.75-7.647c.031-.029.067-.05.098-.08.023-.023.038-.052.06-.076a2.994 2.994 0 0 0-.06-4.166l-9-9A2.99 2.99 0 0 0 12.003 0zM8.63 2.133L.88 9.809a2.998 2.998 0 0 0 0 4.238l7.7 7.75a3.432 3.432 0 0 1-.077-.729 3.432 3.432 0 0 1 3.431-3.431 3.432 3.432 0 0 1 .826.101l-5.523-5.81 4.371-4.373-2.08-2.08c-.903-.904-1.193-2.183-.898-3.342zm3.304 16.004a2.932 2.932 0 0 0-2.931 2.931A2.932 2.932 0 0 0 11.934 24a2.932 2.932 0 0 0 2.932-2.932 2.932 2.932 0 0 0-2.932-2.931z"
        /> < title > { title } < / title > < / svg >
    }
}
