#[cfg(feature = "TiSocialFacebookCircular")]
use leptos::*;
#[cfg(feature = "TiSocialFacebookCircular")]
///This icon requires the feature `TiSocialFacebookCircular` to be enabled.
#[component]
pub fn SocialFacebookCircular(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M18.354 5.624c-1.75-1.741-3.888-2.624-6.354-2.624-2.489 0-4.633.884-6.373 2.625-1.743 1.741-2.627 3.887-2.627 6.375 0 2.465.883 4.603 2.624 6.354 1.741 1.756 3.886 2.646 6.376 2.646 2.467 0 4.605-.89 6.356-2.643 1.755-1.753 2.644-3.892 2.644-6.357 0-2.488-.89-4.634-2.646-6.376zm-1.412 11.319c-1.137 1.139-2.436 1.788-3.942 1.985v-4.928h2v-2h-2v-1.4c0-.331.269-.6.601-.6h1.399v-2h-1.397c-.742 0-1.361.273-1.857.822-.496.547-.746 1.215-.746 2.008v1.17h-2v2h2v4.93c-1.522-.195-2.826-.845-3.957-1.984-1.375-1.384-2.043-3.002-2.043-4.946 0-1.966.667-3.588 2.042-4.96 1.37-1.373 2.992-2.04 4.958-2.04 1.945 0 3.562.668 4.945 2.043 1.383 1.372 2.055 2.994 2.055 4.957 0 1.941-.673 3.559-2.058 4.943z"
        /> < title > { title } < / title > < / svg >
    }
}
