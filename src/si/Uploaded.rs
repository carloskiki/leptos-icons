#[cfg(feature = "SiUploaded")]
use leptos::*;
#[cfg(feature = "SiUploaded")]
///This icon requires the feature `SiUploaded` to be enabled.
#[component]
pub fn Uploaded(
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
        "m14.651 13.862c0 2.5477-2.0653 4.613-4.613 4.613-2.5477 0-4.613-2.0653-4.613-4.613 0-6.1507 9.226-6.1507 9.226 0zm3.497 6.04c-1.33 1.334-3.109 3.957-8.119 4.095-5.451 0.148-9.959-4.484-9.978-10.282-0.01-2.945 0.774-4.659 2.82-6.792 1.224-1.278 7.004-6.923 7.004-6.923 2.052 2.052 0.96 4.873-0.108 5.93l-3.732 3.705c-0.47 0.465-1.968 1.925-1.931 4.444 0.045 3.202 2.51 5.85 5.799 5.8 2.632-0.042 3.756-1.356 4.376-1.98l3.841-3.828c1.103-1.093 3.77-2.027 5.829 5e-3z"
        /> < title > { title } < / title > < / svg >
    }
}
