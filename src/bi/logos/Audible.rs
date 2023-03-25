#[cfg(feature = "BiLogosAudible")]
use leptos::*;
#[cfg(feature = "BiLogosAudible")]
///This icon requires the feature `BiLogosAudible` to be enabled.
#[component]
pub fn Audible(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M2.005 10.238v1.69l10.002 6.254 9.988-6.254v-1.69l-9.988 6.229z" />< path xmlns
        = "http://www.w3.org/2000/svg" d =
        "m15.938 12.469 1.465-.938c-1.161-1.701-3.153-2.876-5.396-2.876-2.257 0-4.236 1.135-5.371 2.89.093-.093.146-.146.238-.211 2.811-2.336 6.86-1.808 9.064 1.135z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M9.051 13.063a2.99 2.99 0 0 1 1.78-.58c1.083 0 2.047.554 2.692 1.49l1.399-.871c-.607-.963-1.688-1.557-2.916-1.557-1.226 0-2.309.62-2.955 1.518zM5.25 9.012c4.117-3.246 9.937-2.362 13.037 1.953l.026.026 1.517-.938a9.337 9.337 0 0 0-7.823-4.235 9.35 9.35 0 0 0-7.825 4.235c.304-.342.686-.751 1.068-1.041z"
        /> < title > { title } < / title > < / svg >
    }
}
