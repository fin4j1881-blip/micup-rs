//! Micup Embed ve Bileşen Oluşturma Makroları.
//!
//! Hızlı kodlama (vibe coding) ve pratik bot geliştirme için pratik bildirim makroları.

#[doc(hidden)]
pub const fn __validate_str_len(s: &str, max: usize) {
    if s.len() > max {
        panic!("Metin uzunlugu izin verilen siniri asiyor!");
    }
}

/// Modal etkileşim yanıtı tür tanımı.
pub type Modal = crate::interaction::InteractionResponse;

/// Tek bildirimle anında `Embed` nesnesi oluşturur.
///
/// Anahtarlar herhangi bir sırada verilebilir.
///
/// # Örnek
/// ```rust
/// use micup::embed;
///
/// let e = embed! {
///     title: "Hoş Geldiniz",
///     description: "Micup botuna hoş geldiniz!",
///     color: 0x7c5cff,
///     fields: [
///         ("Durum", "Aktif", true),
///         ("Geliştirici", "Kerem"),
///     ],
/// };
/// ```
#[macro_export]
macro_rules! embed {
    () => {
        $crate::embed::Embed::new()
    };

    (@step $eb:ident $(,)?) => {};

    (@step $eb:ident, title: $val:literal $(, $($rest:tt)*)?) => {
        const _: () = $crate::macros::__validate_str_len($val, 256);
        $eb = $eb.title($val);
        $crate::embed!(@step $eb $(, $($rest)*)?);
    };
    (@step $eb:ident, title: $val:expr $(, $($rest:tt)*)?) => {
        $crate::macros::__validate_str_len(::std::convert::AsRef::<str>::as_ref(&$val), 256);
        $eb = $eb.title($val);
        $crate::embed!(@step $eb $(, $($rest)*)?);
    };

    (@step $eb:ident, description: $val:literal $(, $($rest:tt)*)?) => {
        const _: () = $crate::macros::__validate_str_len($val, 4096);
        $eb = $eb.description($val);
        $crate::embed!(@step $eb $(, $($rest)*)?);
    };
    (@step $eb:ident, description: $val:expr $(, $($rest:tt)*)?) => {
        $crate::macros::__validate_str_len(::std::convert::AsRef::<str>::as_ref(&$val), 4096);
        $eb = $eb.description($val);
        $crate::embed!(@step $eb $(, $($rest)*)?);
    };

    (@step $eb:ident, color: $val:expr $(, $($rest:tt)*)?) => {
        $eb = $eb.color($val);
        $crate::embed!(@step $eb $(, $($rest)*)?);
    };

    (@step $eb:ident, image: $val:expr $(, $($rest:tt)*)?) => {
        $eb = $eb.image($val);
        $crate::embed!(@step $eb $(, $($rest)*)?);
    };

    (@step $eb:ident, thumbnail: $val:expr $(, $($rest:tt)*)?) => {
        $eb = $eb.thumbnail($val);
        $crate::embed!(@step $eb $(, $($rest)*)?);
    };

    (@step $eb:ident, footer: ($txt:literal, $icon:expr) $(, $($rest:tt)*)?) => {
        const _: () = $crate::macros::__validate_str_len($txt, 2048);
        $eb = $eb.footer($txt, Some($icon.into()));
        $crate::embed!(@step $eb $(, $($rest)*)?);
    };
    (@step $eb:ident, footer: ($txt:expr, $icon:expr) $(, $($rest:tt)*)?) => {
        $crate::macros::__validate_str_len(::std::convert::AsRef::<str>::as_ref(&$txt), 2048);
        $eb = $eb.footer($txt, Some($icon.into()));
        $crate::embed!(@step $eb $(, $($rest)*)?);
    };
    (@step $eb:ident, footer: $txt:literal $(, $($rest:tt)*)?) => {
        const _: () = $crate::macros::__validate_str_len($txt, 2048);
        $eb = $eb.footer($txt, None::<String>);
        $crate::embed!(@step $eb $(, $($rest)*)?);
    };
    (@step $eb:ident, footer: $txt:expr $(, $($rest:tt)*)?) => {
        $crate::macros::__validate_str_len(::std::convert::AsRef::<str>::as_ref(&$txt), 2048);
        $eb = $eb.footer($txt, None::<String>);
        $crate::embed!(@step $eb $(, $($rest)*)?);
    };

    (@step $eb:ident, fields: [ $( ($name:expr, $val:expr $(, $inline:expr)? ) ),* $(,)? ] $(, $($rest:tt)*)?) => {
        $(
            $crate::macros::__validate_str_len(::std::convert::AsRef::<str>::as_ref(&$name), 256);
            $crate::macros::__validate_str_len(::std::convert::AsRef::<str>::as_ref(&$val), 1024);
            #[allow(unused_mut, unused_assignments)]
            let mut inline = false;
            $( inline = $inline; )?
            $eb = $eb.field($name, $val, inline);
        )*
        $crate::embed!(@step $eb $(, $($rest)*)?);
    };
    (@step $eb:ident, fields: $fields:expr $(, $($rest:tt)*)?) => {
        for f in $fields {
            $eb = $eb.field(f.name, f.value, f.inline);
        }
        $crate::embed!(@step $eb $(, $($rest)*)?);
    };

    (@step $eb:ident, field: ($name:expr, $val:expr $(, $inline:expr)? ) $(, $($rest:tt)*)?) => {
        $crate::macros::__validate_str_len(::std::convert::AsRef::<str>::as_ref(&$name), 256);
        $crate::macros::__validate_str_len(::std::convert::AsRef::<str>::as_ref(&$val), 1024);
        #[allow(unused_mut, unused_assignments)]
        let mut inline = false;
        $( inline = $inline; )?
        $eb = $eb.field($name, $val, inline);
        $crate::embed!(@step $eb $(, $($rest)*)?);
    };

    ( $($tokens:tt)* ) => {{
        let mut eb = $crate::embed::Embed::new();
        $crate::embed!(@step eb, $($tokens)*);
        eb
    }};
}

/// Tek bildirimle buton bileşeni (`Component::button`) oluşturur.
///
/// # Örnek
/// ```rust
/// use micup::button;
///
/// let b1 = button!("Kabul Et", "btn_accept", Success);
/// let b2 = button!("Reddet", "btn_reject", Danger);
/// let b3 = button!(link: "Micup Sitesi", "https://micup.gg");
/// ```
#[macro_export]
macro_rules! button {
    // Primary
    ($label:literal, $custom_id:literal, Primary) => {{
        const _: () = $crate::macros::__validate_str_len($label, 80);
        const _: () = $crate::macros::__validate_str_len($custom_id, 100);
        $crate::components::Component::button(
            $crate::components::ButtonStyle::Primary,
            $label,
            $custom_id,
        )
    }};
    ($label:expr, $custom_id:literal, Primary) => {{
        $crate::macros::__validate_str_len(::std::convert::AsRef::<str>::as_ref(&$label), 80);
        const _: () = $crate::macros::__validate_str_len($custom_id, 100);
        $crate::components::Component::button(
            $crate::components::ButtonStyle::Primary,
            $label,
            $custom_id,
        )
    }};
    ($label:expr, $custom_id:expr, Primary) => {{
        $crate::macros::__validate_str_len(::std::convert::AsRef::<str>::as_ref(&$label), 80);
        $crate::macros::__validate_str_len(::std::convert::AsRef::<str>::as_ref(&$custom_id), 100);
        $crate::components::Component::button(
            $crate::components::ButtonStyle::Primary,
            $label,
            $custom_id,
        )
    }};

    // Secondary
    ($label:literal, $custom_id:literal, Secondary) => {{
        const _: () = $crate::macros::__validate_str_len($label, 80);
        const _: () = $crate::macros::__validate_str_len($custom_id, 100);
        $crate::components::Component::button(
            $crate::components::ButtonStyle::Secondary,
            $label,
            $custom_id,
        )
    }};
    ($label:expr, $custom_id:literal, Secondary) => {{
        $crate::macros::__validate_str_len(::std::convert::AsRef::<str>::as_ref(&$label), 80);
        const _: () = $crate::macros::__validate_str_len($custom_id, 100);
        $crate::components::Component::button(
            $crate::components::ButtonStyle::Secondary,
            $label,
            $custom_id,
        )
    }};
    ($label:expr, $custom_id:expr, Secondary) => {{
        $crate::macros::__validate_str_len(::std::convert::AsRef::<str>::as_ref(&$label), 80);
        $crate::macros::__validate_str_len(::std::convert::AsRef::<str>::as_ref(&$custom_id), 100);
        $crate::components::Component::button(
            $crate::components::ButtonStyle::Secondary,
            $label,
            $custom_id,
        )
    }};

    // Success
    ($label:literal, $custom_id:literal, Success) => {{
        const _: () = $crate::macros::__validate_str_len($label, 80);
        const _: () = $crate::macros::__validate_str_len($custom_id, 100);
        $crate::components::Component::button(
            $crate::components::ButtonStyle::Success,
            $label,
            $custom_id,
        )
    }};
    ($label:expr, $custom_id:literal, Success) => {{
        $crate::macros::__validate_str_len(::std::convert::AsRef::<str>::as_ref(&$label), 80);
        const _: () = $crate::macros::__validate_str_len($custom_id, 100);
        $crate::components::Component::button(
            $crate::components::ButtonStyle::Success,
            $label,
            $custom_id,
        )
    }};
    ($label:expr, $custom_id:expr, Success) => {{
        $crate::macros::__validate_str_len(::std::convert::AsRef::<str>::as_ref(&$label), 80);
        $crate::macros::__validate_str_len(::std::convert::AsRef::<str>::as_ref(&$custom_id), 100);
        $crate::components::Component::button(
            $crate::components::ButtonStyle::Success,
            $label,
            $custom_id,
        )
    }};

    // Danger
    ($label:literal, $custom_id:literal, Danger) => {{
        const _: () = $crate::macros::__validate_str_len($label, 80);
        const _: () = $crate::macros::__validate_str_len($custom_id, 100);
        $crate::components::Component::button(
            $crate::components::ButtonStyle::Danger,
            $label,
            $custom_id,
        )
    }};
    ($label:expr, $custom_id:literal, Danger) => {{
        $crate::macros::__validate_str_len(::std::convert::AsRef::<str>::as_ref(&$label), 80);
        const _: () = $crate::macros::__validate_str_len($custom_id, 100);
        $crate::components::Component::button(
            $crate::components::ButtonStyle::Danger,
            $label,
            $custom_id,
        )
    }};
    ($label:expr, $custom_id:expr, Danger) => {{
        $crate::macros::__validate_str_len(::std::convert::AsRef::<str>::as_ref(&$label), 80);
        $crate::macros::__validate_str_len(::std::convert::AsRef::<str>::as_ref(&$custom_id), 100);
        $crate::components::Component::button(
            $crate::components::ButtonStyle::Danger,
            $label,
            $custom_id,
        )
    }};

    // Link
    ($label:expr, $url:expr, Link) => {{
        $crate::macros::__validate_str_len(::std::convert::AsRef::<str>::as_ref(&$label), 80);
        $crate::components::Component::link_button($label, $url)
    }};

    // Disabled varyasyonları
    ($label:expr, $custom_id:expr, Primary, disabled) => {{
        $crate::button!($label, $custom_id, Primary).disabled(true)
    }};
    ($label:expr, $custom_id:expr, Secondary, disabled) => {{
        $crate::button!($label, $custom_id, Secondary).disabled(true)
    }};
    ($label:expr, $custom_id:expr, Success, disabled) => {{
        $crate::button!($label, $custom_id, Success).disabled(true)
    }};
    ($label:expr, $custom_id:expr, Danger, disabled) => {{
        $crate::button!($label, $custom_id, Danger).disabled(true)
    }};
    ($label:expr, $custom_id:expr, $style:expr, disabled) => {{
        $crate::button!($label, $custom_id, $style).disabled(true)
    }};
    ($label:expr, $custom_id:expr, disabled) => {{
        $crate::button!($label, $custom_id, Primary).disabled(true)
    }};

    // Genel stil ifadesi (ör. ButtonStyle::Primary)
    ($label:literal, $custom_id:literal, $style:expr) => {{
        const _: () = $crate::macros::__validate_str_len($label, 80);
        const _: () = $crate::macros::__validate_str_len($custom_id, 100);
        $crate::components::Component::button($style, $label, $custom_id)
    }};
    ($label:expr, $custom_id:literal, $style:expr) => {{
        $crate::macros::__validate_str_len(::std::convert::AsRef::<str>::as_ref(&$label), 80);
        const _: () = $crate::macros::__validate_str_len($custom_id, 100);
        $crate::components::Component::button($style, $label, $custom_id)
    }};
    ($label:expr, $custom_id:expr, $style:expr) => {{
        $crate::macros::__validate_str_len(::std::convert::AsRef::<str>::as_ref(&$label), 80);
        $crate::macros::__validate_str_len(::std::convert::AsRef::<str>::as_ref(&$custom_id), 100);
        $crate::components::Component::button($style, $label, $custom_id)
    }};

    // Varsayılan stil (Primary)
    ($label:literal, $custom_id:literal) => {{
        const _: () = $crate::macros::__validate_str_len($label, 80);
        const _: () = $crate::macros::__validate_str_len($custom_id, 100);
        $crate::components::Component::button(
            $crate::components::ButtonStyle::Primary,
            $label,
            $custom_id,
        )
    }};
    ($label:expr, $custom_id:literal) => {{
        $crate::macros::__validate_str_len(::std::convert::AsRef::<str>::as_ref(&$label), 80);
        const _: () = $crate::macros::__validate_str_len($custom_id, 100);
        $crate::components::Component::button(
            $crate::components::ButtonStyle::Primary,
            $label,
            $custom_id,
        )
    }};
    ($label:expr, $custom_id:expr) => {{
        $crate::macros::__validate_str_len(::std::convert::AsRef::<str>::as_ref(&$label), 80);
        $crate::macros::__validate_str_len(::std::convert::AsRef::<str>::as_ref(&$custom_id), 100);
        $crate::components::Component::button(
            $crate::components::ButtonStyle::Primary,
            $label,
            $custom_id,
        )
    }};

    // Link buton sözdizimi
    (link: $label:expr, $url:expr) => {{
        $crate::macros::__validate_str_len(::std::convert::AsRef::<str>::as_ref(&$label), 80);
        $crate::components::Component::link_button($label, $url)
    }};
    ($label:expr, link: $url:expr) => {{
        $crate::macros::__validate_str_len(::std::convert::AsRef::<str>::as_ref(&$label), 80);
        $crate::components::Component::link_button($label, $url)
    }};

    // İsimli önek sözdizimi
    (primary: $label:expr, $custom_id:expr) => {{
        $crate::button!($label, $custom_id, Primary)
    }};
    (secondary: $label:expr, $custom_id:expr) => {{
        $crate::button!($label, $custom_id, Secondary)
    }};
    (success: $label:expr, $custom_id:expr) => {{
        $crate::button!($label, $custom_id, Success)
    }};
    (danger: $label:expr, $custom_id:expr) => {{
        $crate::button!($label, $custom_id, Danger)
    }};
}

/// Tek bildirimle `ActionRow` bileşeni (`Component::action_row`) oluşturur.
///
/// ActionRow en fazla 5 bileşen içerebilir.
/// Hem `Component` hem de `Button`, `SelectMenu`, `TextInput` gibi doğrudan modelleri kabul eder.
///
/// # Örnek
/// ```rust
/// use micup::prelude::*;
///
/// let b1 = button!("Kabul Et", "btn_accept", Success);
/// let row = action_row![b1];
/// ```
#[macro_export]
macro_rules! action_row {
    ( $( $comp:expr ),* $(,)? ) => {{
        const COUNT: usize = [ $( { let _ = stringify!($comp); } ),* ].len();
        const _: () = {
            if COUNT > 5 {
                panic!("ActionRow en fazla 5 bileşen içerebilir!");
            }
        };
        $crate::components::Component::action_row(vec![ $( $crate::components::Component::from($comp) ),* ])
    }};
}

/// Tek bildirimle bileşen satırı (`Component::action_row`) oluşturur.
///
/// ActionRow en fazla 5 bileşen içerebilir.
///
/// # Örnek
/// ```rust
/// use micup::{button, component_row};
///
/// let b1 = button!("Kabul Et", "btn_accept", Success);
/// let row = component_row![b1];
/// ```
#[macro_export]
macro_rules! component_row {
    ( $( $comp:expr ),* $(,)? ) => {{
        $crate::action_row!( $( $comp ),* )
    }};
}

/// Tek bildirimle metin girişi kutusu (`Component::text_input`) oluşturur.
///
/// # Örnek
/// ```rust
/// use micup::text_input;
///
/// let i1 = text_input!("txt_ad", "Adınız");
/// let i2 = text_input!("txt_mesaj", Paragraph, "Mesajınız");
/// ```
#[macro_export]
macro_rules! text_input {
    ($custom_id:literal, Short, $label:expr) => {{
        const _: () = $crate::macros::__validate_str_len($custom_id, 100);
        $crate::components::Component::text_input(
            $custom_id,
            $crate::components::TextInputStyle::Short,
            $label,
        )
    }};
    ($custom_id:expr, Short, $label:expr) => {{
        $crate::macros::__validate_str_len(::std::convert::AsRef::<str>::as_ref(&$custom_id), 100);
        $crate::components::Component::text_input(
            $custom_id,
            $crate::components::TextInputStyle::Short,
            $label,
        )
    }};
    ($custom_id:literal, Paragraph, $label:expr) => {{
        const _: () = $crate::macros::__validate_str_len($custom_id, 100);
        $crate::components::Component::text_input(
            $custom_id,
            $crate::components::TextInputStyle::Paragraph,
            $label,
        )
    }};
    ($custom_id:expr, Paragraph, $label:expr) => {{
        $crate::macros::__validate_str_len(::std::convert::AsRef::<str>::as_ref(&$custom_id), 100);
        $crate::components::Component::text_input(
            $custom_id,
            $crate::components::TextInputStyle::Paragraph,
            $label,
        )
    }};
    ($custom_id:literal, $style:expr, $label:expr) => {{
        const _: () = $crate::macros::__validate_str_len($custom_id, 100);
        $crate::components::Component::text_input($custom_id, $style, $label)
    }};
    ($custom_id:expr, $style:expr, $label:expr) => {{
        $crate::macros::__validate_str_len(::std::convert::AsRef::<str>::as_ref(&$custom_id), 100);
        $crate::components::Component::text_input($custom_id, $style, $label)
    }};
    ($custom_id:literal, $label:expr) => {{
        const _: () = $crate::macros::__validate_str_len($custom_id, 100);
        $crate::components::Component::text_input(
            $custom_id,
            $crate::components::TextInputStyle::Short,
            $label,
        )
    }};
    ($custom_id:expr, $label:expr) => {{
        $crate::macros::__validate_str_len(::std::convert::AsRef::<str>::as_ref(&$custom_id), 100);
        $crate::components::Component::text_input(
            $custom_id,
            $crate::components::TextInputStyle::Short,
            $label,
        )
    }};
}

/// Tek bildirimle seçim menüsü bileşeni (`Component::string_select`) oluşturur.
///
/// # Örnek
/// ```rust
/// use micup::prelude::*;
///
/// let menu = select_menu! {
///     custom_id: "secim_menusu",
///     placeholder: "Bir secenek secin",
///     options: [
///         ("Secenek 1", "opt_1"),
///         ("Secenek 2", "opt_2", "Aciklama 2"),
///     ],
/// };
/// ```
#[macro_export]
macro_rules! select_menu {
    (
        custom_id: $custom_id:literal,
        $(placeholder: $placeholder:expr,)?
        options: [ $( ($label:expr, $val:expr $(, $desc:expr)? ) ),* $(,)? ] $(,)?
    ) => {{
        const _: () = $crate::macros::__validate_str_len($custom_id, 100);
        const OPT_COUNT: usize = [ $( { let _ = stringify!($label); } ),* ].len();
        const _: () = {
            if OPT_COUNT > 25 {
                panic!("Seçim menüsü en fazla 25 seçenek içerebilir!");
            }
        };
        let mut opts = Vec::new();
        $(
            #[allow(unused_mut)]
            let mut opt = $crate::components::SelectOption::new($label, $val);
            $( opt = opt.description($desc); )?
            opts.push(opt);
        )*
        #[allow(unused_mut, unused_assignments)]
        let mut ph: Option<&str> = None;
        $( ph = Some($placeholder); )?
        $crate::components::Component::string_select($custom_id, ph, opts)
    }};
    (
        custom_id: $custom_id:expr,
        $(placeholder: $placeholder:expr,)?
        options: [ $( ($label:expr, $val:expr $(, $desc:expr)? ) ),* $(,)? ] $(,)?
    ) => {{
        $crate::macros::__validate_str_len(::std::convert::AsRef::<str>::as_ref(&$custom_id), 100);
        const OPT_COUNT: usize = [ $( { let _ = stringify!($label); } ),* ].len();
        const _: () = {
            if OPT_COUNT > 25 {
                panic!("Seçim menüsü en fazla 25 seçenek içerebilir!");
            }
        };
        let mut opts = Vec::new();
        $(
            #[allow(unused_mut)]
            let mut opt = $crate::components::SelectOption::new($label, $val);
            $( opt = opt.description($desc); )?
            opts.push(opt);
        )*
        #[allow(unused_mut, unused_assignments)]
        let mut ph: Option<&str> = None;
        $( ph = Some($placeholder); )?
        $crate::components::Component::string_select($custom_id, ph, opts)
    }};
    (
        placeholder: $placeholder:expr,
        custom_id: $custom_id:expr,
        options: [ $( ($label:expr, $val:expr $(, $desc:expr)? ) ),* $(,)? ] $(,)?
    ) => {{
        $crate::select_menu! {
            custom_id: $custom_id,
            placeholder: $placeholder,
            options: [ $( ($label, $val $(, $desc)? ) ),* ],
        }
    }};
    (
        custom_id: $custom_id:expr,
        $(placeholder: $placeholder:expr,)?
        options: $options:expr $(,)?
    ) => {{
        $crate::macros::__validate_str_len(::std::convert::AsRef::<str>::as_ref(&$custom_id), 100);
        #[allow(unused_mut, unused_assignments)]
        let mut ph: Option<&str> = None;
        $( ph = Some($placeholder); )?
        $crate::components::Component::string_select($custom_id, ph, $options)
    }};
    (
        $custom_id:expr,
        $placeholder:expr,
        [ $( ($label:expr, $val:expr $(, $desc:expr)? ) ),* $(,)? ] $(,)?
    ) => {{
        $crate::select_menu! {
            custom_id: $custom_id,
            placeholder: $placeholder,
            options: [ $( ($label, $val $(, $desc)? ) ),* ],
        }
    }};
    (
        $custom_id:expr,
        options: [ $( ($label:expr, $val:expr $(, $desc:expr)? ) ),* $(,)? ] $(,)?
    ) => {{
        $crate::select_menu! {
            custom_id: $custom_id,
            options: [ $( ($label, $val $(, $desc)? ) ),* ],
        }
    }};
    (
        $custom_id:expr,
        [ $( ($label:expr, $val:expr $(, $desc:expr)? ) ),* $(,)? ] $(,)?
    ) => {{
        $crate::select_menu! {
            custom_id: $custom_id,
            options: [ $( ($label, $val $(, $desc)? ) ),* ],
        }
    }};
}

/// Tek bildirimle modal (açılır form) yanıtı (`InteractionResponse::modal`) oluşturur.
///
/// # Örnek
/// ```rust
/// use micup::modal;
///
/// let m = modal! {
///     title: "Geri Bildirim Formu",
///     custom_id: "modal_feedback",
///     components: [],
/// };
/// ```
#[macro_export]
macro_rules! modal {
    (
        title: $title:expr,
        custom_id: $custom_id:literal,
        components: [ $( $comp:expr ),* $(,)? ] $(,)?
    ) => {{
        const _: () = $crate::macros::__validate_str_len($custom_id, 100);
        $crate::interaction::InteractionResponse::modal(
            $custom_id,
            $title,
            vec![ $( $crate::components::Component::from($comp) ),* ],
        )
    }};
    (
        title: $title:expr,
        custom_id: $custom_id:expr,
        components: [ $( $comp:expr ),* $(,)? ] $(,)?
    ) => {{
        $crate::macros::__validate_str_len(::std::convert::AsRef::<str>::as_ref(&$custom_id), 100);
        $crate::interaction::InteractionResponse::modal(
            $custom_id,
            $title,
            vec![ $( $crate::components::Component::from($comp) ),* ],
        )
    }};
    (
        custom_id: $custom_id:literal,
        title: $title:expr,
        components: [ $( $comp:expr ),* $(,)? ] $(,)?
    ) => {{
        const _: () = $crate::macros::__validate_str_len($custom_id, 100);
        $crate::interaction::InteractionResponse::modal(
            $custom_id,
            $title,
            vec![ $( $crate::components::Component::from($comp) ),* ],
        )
    }};
    (
        custom_id: $custom_id:expr,
        title: $title:expr,
        components: [ $( $comp:expr ),* $(,)? ] $(,)?
    ) => {{
        $crate::macros::__validate_str_len(::std::convert::AsRef::<str>::as_ref(&$custom_id), 100);
        $crate::interaction::InteractionResponse::modal(
            $custom_id,
            $title,
            vec![ $( $crate::components::Component::from($comp) ),* ],
        )
    }};
    (
        title: $title:expr,
        custom_id: $custom_id:literal,
        components: $components:expr $(,)?
    ) => {{
        const _: () = $crate::macros::__validate_str_len($custom_id, 100);
        $crate::interaction::InteractionResponse::modal(
            $custom_id,
            $title,
            $components,
        )
    }};
    (
        title: $title:expr,
        custom_id: $custom_id:expr,
        components: $components:expr $(,)?
    ) => {{
        $crate::macros::__validate_str_len(::std::convert::AsRef::<str>::as_ref(&$custom_id), 100);
        $crate::interaction::InteractionResponse::modal(
            $custom_id,
            $title,
            $components,
        )
    }};
    (
        custom_id: $custom_id:literal,
        title: $title:expr,
        components: $components:expr $(,)?
    ) => {{
        const _: () = $crate::macros::__validate_str_len($custom_id, 100);
        $crate::interaction::InteractionResponse::modal(
            $custom_id,
            $title,
            $components,
        )
    }};
    (
        custom_id: $custom_id:expr,
        title: $title:expr,
        components: $components:expr $(,)?
    ) => {{
        $crate::macros::__validate_str_len(::std::convert::AsRef::<str>::as_ref(&$custom_id), 100);
        $crate::interaction::InteractionResponse::modal(
            $custom_id,
            $title,
            $components,
        )
    }};
    (
        title: $title:expr,
        custom_id: $custom_id:literal $(,)?
    ) => {{
        const _: () = $crate::macros::__validate_str_len($custom_id, 100);
        $crate::interaction::InteractionResponse::modal(
            $custom_id,
            $title,
            ::std::vec::Vec::new(),
        )
    }};
    (
        title: $title:expr,
        custom_id: $custom_id:expr $(,)?
    ) => {{
        $crate::macros::__validate_str_len(::std::convert::AsRef::<str>::as_ref(&$custom_id), 100);
        $crate::interaction::InteractionResponse::modal(
            $custom_id,
            $title,
            ::std::vec::Vec::new(),
        )
    }};
    (
        custom_id: $custom_id:literal,
        title: $title:expr $(,)?
    ) => {{
        const _: () = $crate::macros::__validate_str_len($custom_id, 100);
        $crate::interaction::InteractionResponse::modal(
            $custom_id,
            $title,
            ::std::vec::Vec::new(),
        )
    }};
    (
        custom_id: $custom_id:expr,
        title: $title:expr $(,)?
    ) => {{
        $crate::macros::__validate_str_len(::std::convert::AsRef::<str>::as_ref(&$custom_id), 100);
        $crate::interaction::InteractionResponse::modal(
            $custom_id,
            $title,
            ::std::vec::Vec::new(),
        )
    }};
    (
        $custom_id:literal,
        $title:expr,
        [ $( $comp:expr ),* $(,)? ] $(,)?
    ) => {{
        const _: () = $crate::macros::__validate_str_len($custom_id, 100);
        $crate::interaction::InteractionResponse::modal(
            $custom_id,
            $title,
            vec![ $( $crate::components::Component::from($comp) ),* ],
        )
    }};
    (
        $custom_id:expr,
        $title:expr,
        [ $( $comp:expr ),* $(,)? ] $(,)?
    ) => {{
        $crate::macros::__validate_str_len(::std::convert::AsRef::<str>::as_ref(&$custom_id), 100);
        $crate::interaction::InteractionResponse::modal(
            $custom_id,
            $title,
            vec![ $( $crate::components::Component::from($comp) ),* ],
        )
    }};
    (
        $custom_id:literal,
        $title:expr,
        $components:expr $(,)?
    ) => {{
        const _: () = $crate::macros::__validate_str_len($custom_id, 100);
        $crate::interaction::InteractionResponse::modal(
            $custom_id,
            $title,
            $components,
        )
    }};
    (
        $custom_id:expr,
        $title:expr,
        $components:expr $(,)?
    ) => {{
        $crate::macros::__validate_str_len(::std::convert::AsRef::<str>::as_ref(&$custom_id), 100);
        $crate::interaction::InteractionResponse::modal(
            $custom_id,
            $title,
            $components,
        )
    }};
    (
        $custom_id:literal,
        $title:expr $(,)?
    ) => {{
        const _: () = $crate::macros::__validate_str_len($custom_id, 100);
        $crate::interaction::InteractionResponse::modal(
            $custom_id,
            $title,
            ::std::vec::Vec::new(),
        )
    }};
    (
        $custom_id:expr,
        $title:expr $(,)?
    ) => {{
        $crate::macros::__validate_str_len(::std::convert::AsRef::<str>::as_ref(&$custom_id), 100);
        $crate::interaction::InteractionResponse::modal(
            $custom_id,
            $title,
            ::std::vec::Vec::new(),
        )
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::ButtonStyle;

    #[test]
    #[should_panic(expected = "Metin uzunlugu izin verilen siniri asiyor!")]
    fn test_validate_str_len_embed_title_exceeded() {
        __validate_str_len(&"a".repeat(257), 256);
    }

    #[test]
    #[should_panic(expected = "Metin uzunlugu izin verilen siniri asiyor!")]
    fn test_validate_str_len_embed_description_exceeded() {
        __validate_str_len(&"a".repeat(4097), 4096);
    }

    #[test]
    #[should_panic(expected = "Metin uzunlugu izin verilen siniri asiyor!")]
    fn test_validate_str_len_embed_field_exceeded() {
        __validate_str_len(&"a".repeat(1025), 1024);
    }

    #[test]
    #[should_panic(expected = "Metin uzunlugu izin verilen siniri asiyor!")]
    fn test_validate_str_len_button_custom_id_exceeded() {
        __validate_str_len(&"a".repeat(101), 100);
    }

    #[test]
    fn test_validate_str_len_valid() {
        __validate_str_len("Gecerli Baslik", 256);
        __validate_str_len("btn_id", 100);
    }

    #[test]
    fn test_embed_macro_flexible_order_and_dynamic() {
        let dyn_title = "Dinamik Başlık".to_string();
        let e = embed! {
            color: 0x123456,
            description: "Açıklama önce geldi",
            title: &dyn_title,
            image: "https://media.micup.gg/img.png",
            thumbnail: "https://media.micup.gg/thumb.png",
            footer: "Basit altbilgi",
            fields: [
                ("Alan 1", "Değer 1", true),
                ("Alan 2", "Değer 2"),
            ],
            field: ("Alan 3", "Değer 3", false),
        };
        assert_eq!(e.title.as_deref(), Some("Dinamik Başlık"));
        assert_eq!(e.color, Some(0x123456));
        assert_eq!(e.description.as_deref(), Some("Açıklama önce geldi"));
        assert_eq!(e.fields.len(), 3);
        assert!(e.fields[0].inline);
        assert!(!e.fields[1].inline);
        assert_eq!(
            e.footer.as_ref().map(|f| f.text.as_str()),
            Some("Basit altbilgi")
        );

        let e_footer_tuple = embed! {
            footer: ("Detaylı Altbilgi", "https://media.micup.gg/icon.png"),
        };
        assert_eq!(
            e_footer_tuple.footer.as_ref().map(|f| f.text.as_str()),
            Some("Detaylı Altbilgi")
        );
        assert_eq!(
            e_footer_tuple
                .footer
                .as_ref()
                .and_then(|f| f.icon_url.as_deref()),
            Some("https://media.micup.gg/icon.png")
        );
    }

    #[test]
    fn test_embed_fields_vec_expression() {
        let fields = vec![
            crate::embed::EmbedField {
                name: "F1".into(),
                value: "V1".into(),
                inline: true,
            },
            crate::embed::EmbedField {
                name: "F2".into(),
                value: "V2".into(),
                inline: false,
            },
        ];
        let e = embed! {
            title: "Liste",
            fields: fields,
        };
        assert_eq!(e.fields.len(), 2);
    }

    #[test]
    fn test_button_macro_variations() {
        let b1 = button!("Primary", "btn_1", Primary);
        let b2 = button!("Secondary", "btn_2", Secondary);
        let b3 = button!("Success", "btn_3", Success);
        let b4 = button!("Danger", "btn_4", Danger);
        let b5 = button!("Default", "btn_5");
        let b6 = button!("Link", "https://micup.gg", Link);
        let b7 = button!("StyleExpr", "btn_7", ButtonStyle::Danger);
        let b8 = button!(link: "Link2", "https://micup.gg");
        let b9 = button!("Link3", link: "https://micup.gg");
        let b10 = button!(primary: "Pri", "btn_pri");
        let b11 = button!(danger: "Dan", "btn_dan");
        let b12 = button!("Disabled", "btn_dis", disabled);

        let j1 = serde_json::to_value(&b1).unwrap();
        assert_eq!(j1["style"], 1);
        let j2 = serde_json::to_value(&b2).unwrap();
        assert_eq!(j2["style"], 2);
        let j3 = serde_json::to_value(&b3).unwrap();
        assert_eq!(j3["style"], 3);
        let j4 = serde_json::to_value(&b4).unwrap();
        assert_eq!(j4["style"], 4);
        let j5 = serde_json::to_value(&b5).unwrap();
        assert_eq!(j5["style"], 1);
        let j6 = serde_json::to_value(&b6).unwrap();
        assert_eq!(j6["style"], 5);
        assert_eq!(j6["url"], "https://micup.gg");
        let j7 = serde_json::to_value(&b7).unwrap();
        assert_eq!(j7["style"], 4);
        let j8 = serde_json::to_value(&b8).unwrap();
        assert_eq!(j8["style"], 5);
        let j9 = serde_json::to_value(&b9).unwrap();
        assert_eq!(j9["style"], 5);
        let j10 = serde_json::to_value(&b10).unwrap();
        assert_eq!(j10["style"], 1);
        let j11 = serde_json::to_value(&b11).unwrap();
        assert_eq!(j11["style"], 4);
        let j12 = serde_json::to_value(&b12).unwrap();
        assert_eq!(j12["disabled"], true);
    }

    #[test]
    fn test_component_row_macro() {
        let b1 = button!("Kabul", "btn_accept", Success);
        let b2 = button!("Red", "btn_reject", Danger);
        let row = component_row![b1, b2];
        let json = serde_json::to_value(&row).expect("Serileştirme başarılı olmalı");
        assert_eq!(json["type"], 1);
        let comps = json["components"]
            .as_array()
            .expect("Bileşen dizisi olmalı");
        assert_eq!(comps.len(), 2);
    }

    #[test]
    fn test_action_row_with_button_model() {
        let b = crate::components::Button::primary("Test", "btn_test");
        let row = action_row![b];
        let json = serde_json::to_value(&row).unwrap();
        assert_eq!(json["type"], 1);
        assert_eq!(json["components"].as_array().unwrap().len(), 1);
    }

    #[test]
    fn test_text_input_macro() {
        let i1 = text_input!("txt_name", "Adınız");
        let i2 = text_input!("txt_bio", Paragraph, "Biyografi");
        let row = action_row![i1, i2];
        let json = serde_json::to_value(&row).unwrap();
        assert_eq!(json["type"], 1);
        let comps = json["components"].as_array().unwrap();
        assert_eq!(comps[0]["custom_id"], "txt_name");
        assert_eq!(comps[0]["style"], 1);
        assert_eq!(comps[1]["custom_id"], "txt_bio");
        assert_eq!(comps[1]["style"], 2);
    }

    #[test]
    fn test_select_menu_macro() {
        let menu1 = select_menu! {
            custom_id: "secim_1",
            placeholder: "Secim yapin",
            options: [
                ("A", "val_a"),
                ("B", "val_b", "Detay B"),
            ],
        };
        let j1 = serde_json::to_value(&menu1).unwrap();
        assert_eq!(j1["type"], 3);
        assert_eq!(j1["custom_id"], "secim_1");
        assert_eq!(j1["placeholder"], "Secim yapin");
        assert_eq!(j1["options"].as_array().unwrap().len(), 2);

        let menu2 = select_menu!("secim_2", "Lütfen seçin", [("Opt1", "1"), ("Opt2", "2"),]);
        let j2 = serde_json::to_value(&menu2).unwrap();
        assert_eq!(j2["custom_id"], "secim_2");
        assert_eq!(j2["placeholder"], "Lütfen seçin");
    }

    #[test]
    fn test_modal_macro() {
        let m = modal! {
            title: "Geri Bildirim Formu",
            custom_id: "modal_fb",
            components: [
                component_row![
                    text_input!("txt_1", Paragraph, "Mesajınız")
                ],
            ],
        };
        let json = serde_json::to_value(&m).expect("Modal serileştirme başarılı olmalı");
        assert_eq!(json["type"], 9);
        assert_eq!(json["data"]["title"], "Geri Bildirim Formu");
        assert_eq!(json["data"]["custom_id"], "modal_fb");
        let comps = json["data"]["components"]
            .as_array()
            .expect("Bileşenler dizisi olmalı");
        assert_eq!(comps.len(), 1);
    }

    #[test]
    fn test_modal_positional_and_custom_id_first() {
        let m1 = modal! {
            custom_id: "m_cid",
            title: "M Title",
            components: [],
        };
        let json1 = serde_json::to_value(&m1).unwrap();
        assert_eq!(json1["type"], 9);
        assert_eq!(json1["data"]["custom_id"], "m_cid");
        assert_eq!(json1["data"]["title"], "M Title");

        let m2 = modal!("pos_id", "Pos Title");
        let json2 = serde_json::to_value(&m2).unwrap();
        assert_eq!(json2["type"], 9);
        assert_eq!(json2["data"]["custom_id"], "pos_id");
        assert_eq!(json2["data"]["title"], "Pos Title");
    }
}
