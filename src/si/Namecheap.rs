#[cfg(feature = "SiNamecheap")]
use leptos::*;
#[cfg(feature = "SiNamecheap")]
///This icon requires the feature `SiNamecheap` to be enabled.
#[component]
pub fn Namecheap(
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
        "M17.295 17.484c.227.403.57.728.985.931-.309.15-.647.229-.99.232h-3.068a2.26 2.26 0 0 1-1.957-1.143L6.705 6.511a2.27 2.27 0 0 0-.974-.922c.309-.153.652-.233.997-.232h3.05c.81.003 1.558.438 1.959 1.143l5.558 10.984zm-9.329-7.392L6.269 6.755c-.209-.392-.582-.657-.984-.829-.204.165-.391.35-.522.581-.184.349-4.391 8.648-4.569 8.987a2.245 2.245 0 0 0 4.016 1.999l3.756-7.401zm15.846-1.593a2.245 2.245 0 0 0-1.162-2.955v-.001a2.243 2.243 0 0 0-.892-.187l-.003-.011c-.816 0-1.569.443-1.965 1.157l-3.749 7.414 1.689 3.323c.213.399.59.664.998.839.252-.2.473-.444.605-.742l4.479-8.837z"
        /> < title > { title } < / title > < / svg >
    }
}
