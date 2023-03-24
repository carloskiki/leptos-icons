#[cfg(feature = "SiCirrusci")]
use leptos::*;
#[cfg(feature = "SiCirrusci")]
///This icon requires the feature `SiCirrusci` to be enabled.
#[component]
pub fn Cirrusci(
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
        "M19.422.453a1.113 1.113 0 0 0-1.113 1.115 1.113 1.113 0 0 0 1.112 1.114c1.31 0 2.35 1.042 2.35 2.363 0 1.32-1.04 2.363-2.35 2.363H1.112A1.113 1.113 0 0 0 0 8.52a1.113 1.113 0 0 0 1.113 1.117h18.31c1.308 0 2.35 1.042 2.35 2.363 0 1.32-1.042 2.363-2.35 2.363H1.112A1.113 1.113 0 0 0 0 15.48a1.113 1.113 0 0 0 1.113 1.112h18.31c1.308 0 2.35 1.042 2.35 2.363 0 1.32-1.042 2.363-2.35 2.363H1.112A1.113 1.113 0 0 0 0 22.432a1.113 1.113 0 0 0 1.113 1.115h18.31a1.113 1.113 0 0 0 .206-.022c2.42-.112 4.37-2.12 4.37-4.57 0-1.393-.642-2.634-1.63-3.478C23.356 14.632 24 13.393 24 12c0-1.393-.643-2.632-1.63-3.477C23.357 7.68 24 6.438 24 5.045c0-2.52-2.06-4.592-4.578-4.592z"
        /> < title > { title } < / title > < / svg >
    }
}
