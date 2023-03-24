#[cfg(feature = "OcLgTools")]
use leptos::*;
#[cfg(feature = "OcLgTools")]
///This icon requires the feature `OcLgTools` to be enabled.
#[component]
pub fn Tools(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M7.875 2.292a.114.114 0 0 0-.032.018A7.239 7.239 0 0 0 4.75 8.25a7.248 7.248 0 0 0 3.654 6.297c.57.327.982.955.941 1.682v.002l-.317 6.058a.75.75 0 1 1-1.498-.078l.317-6.062v-.004c.006-.09-.047-.215-.188-.296A8.749 8.749 0 0 1 3.25 8.25a8.738 8.738 0 0 1 3.732-7.169 1.547 1.547 0 0 1 1.709-.064c.484.292.809.835.809 1.46v4.714a.25.25 0 0 0 .119.213l2.25 1.385c.08.05.182.05.262 0l2.25-1.385a.25.25 0 0 0 .119-.213V2.478c0-.626.325-1.169.81-1.461a1.547 1.547 0 0 1 1.708.064 8.741 8.741 0 0 1 3.732 7.17 8.747 8.747 0 0 1-4.41 7.598c-.14.081-.193.206-.188.296v.004l.318 6.062a.75.75 0 1 1-1.498.078l-.317-6.058v-.002c-.041-.727.37-1.355.94-1.682A7.247 7.247 0 0 0 19.25 8.25a7.239 7.239 0 0 0-3.093-5.94.114.114 0 0 0-.032-.018l-.01-.001c-.003 0-.014 0-.031.01-.036.022-.084.079-.084.177V7.19c0 .608-.315 1.172-.833 1.49l-2.25 1.385a1.75 1.75 0 0 1-1.834 0l-2.25-1.384A1.752 1.752 0 0 1 8 7.192V2.477c0-.098-.048-.155-.084-.176a.068.068 0 0 0-.031-.011l-.01.001Z"
        /> < title > { title } < / title > < / svg >
    }
}
