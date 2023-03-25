#[cfg(feature = "SiBloglovin")]
use leptos::*;
#[cfg(feature = "SiBloglovin")]
///This icon requires the feature `SiBloglovin` to be enabled.
#[component]
pub fn Bloglovin(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M12.526 11.695c1.84-.382 3.367-2.044 3.367-4.478 0-2.604-1.9-4.97-5.615-4.97H0v19.506h10.6c3.75 0 5.683-2.341 5.683-5.292-.009-2.426-1.646-4.444-3.757-4.766zm-8.37-5.793h5.207c1.407 0 2.28.849 2.28 2.044 0 1.255-.881 2.044-2.28 2.044H4.155zM9.54 18.098H4.155v-4.444h5.386c1.61 0 2.484.992 2.484 2.222.009 1.399-.932 2.222-2.484 2.222zM21.396 2.28c-1.255 0-2.315 1.052-2.315 2.307s.882 2.103 1.993 2.103c.238 0 .467-.025.56-.085-.238 1.052-1.315 2.282-2.256 2.782l1.611 1.314C22.796 9.422 24 7.462 24 5.266c0-1.9-1.23-2.985-2.604-2.985Z"
        /> < title > { title } < / title > < / svg >
    }
}
