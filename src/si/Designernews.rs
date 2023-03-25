#[cfg(feature = "SiDesignernews")]
use leptos::*;
#[cfg(feature = "SiDesignernews")]
///This icon requires the feature `SiDesignernews` to be enabled.
#[component]
pub fn Designernews(
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
        "M11.27 11.98c0-3.83-2.354-6.43-6.84-6.43H0v12.9h4.524c4.354 0 6.747-2.624 6.747-6.464v-.005zM8.056 12c0 2.766-1.42 3.963-3.7 3.963h-1.16V8.037h1.16c2.185 0 3.7 1.252 3.7 3.963zM24 18.45V5.55h-2.97v7.213L16.28 5.55h-3.105v12.9h2.973v-7.723l5.084 7.718H24v.004z"
        /> < title > { title } < / title > < / svg >
    }
}
