#[cfg(feature = "SiArdour")]
use leptos::*;
#[cfg(feature = "SiArdour")]
///This icon requires the feature `SiArdour` to be enabled.
#[component]
pub fn Ardour(
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
        "M12 1.606L0 22.394c1.45 0 .832-.885 1.565-.93.703 0 .559.44 1.044.44.846 0 .273-2.82 1.043-2.82.665 0 .48 2.038 1.044 2.038.288 0 .521-.811.521-1.81v-.945c0-1.304.234-2.364.522-2.364.288 0 .522 1.066.522 2.005 0 1.178.233 2.43.522 2.43.288 0 .521-1.263.521-2.805v-.44c0-1.69.234-3.065.522-3.065.288 0 .522 1.369.522 2.967 0 1.661.233 3.098.522 3.098.288 0 .521-1.437.521-3.18 0-1.737.234-3.146.522-3.146.288 0 .522 1.424.522 3.277 0 1.786.233 3.147.522 3.147.288 0 .521-1.367.521-2.87 0-1.386.234-2.657.522-2.657.288 0 .522 1.271.522 2.837v.472c0 1.415.233 2.56.521 2.56.289 0 .522-1.152.522-2.299 0-.973.234-1.989.522-1.989.288 0 .522 1.01.522 2.25v.57c0 1.058.233 1.908.521 1.908.289 0 .522-.84.522-1.614 0-.589.234-1.304.522-1.304.288 0 .522.709.522 1.581v.538c0 .696.233 1.272.521 1.272.595 0 .45-1.728 1.044-1.728.288 0 .522.43.522.962v.456c0 .385.233.685.521.685.59 0 .462-.782 1.044-.782.76 0 .197 1.076 1.043 1.076.512 0 .426-.18 1.044-.18.563 0 .493.359 1.565.359z"
        /> < title > { title } < / title > < / svg >
    }
}
