//! Guided setup for the optional Spotify player, inside Settings.
use crate::{app::App, i18n::gettext, model::Action, soloist::Status, theme};

pub fn show(app: &mut App, ui: &mut egui::Ui) {
    let locale = app.locale;
    let palette = app.palette;
    ui.label(gettext(locale, "Spotify's own player, with support for lossless audio. Requires Premium, your personal Soloist key, and one pairing in the Spotify app."));
    ui.add_space(10.0);
    ui.label(egui::RichText::new(gettext(locale, "Soloist plays through your system audio output. Spotifast's EQ, visualizers, local files, and full queue editing are available with the built-in player." )).color(palette.secondary));
    ui.add_space(12.0);
    if !crate::soloist::install::supported() {
        ui.label(gettext(locale, "Spotify has not published Soloist for this platform. The built-in player remains available."));
        if app.settings.soloist_enabled
            && theme::soft_button(
                ui,
                &palette,
                None,
                &gettext(locale, "Turn off Soloist"),
                false,
            )
            .clicked()
        {
            app.actions.push(Action::SetSoloist(false));
        }
        return;
    }
    if !app.is_connected() {
        ui.label(gettext(
            locale,
            "Sign in to Spotify before setting up Soloist.",
        ));
        return;
    }
    match &app.soloist_status {
        Status::Off | Status::NeedsKey | Status::Failed(_) => {
            if let Status::Failed(error) = &app.soloist_status {
                ui.label(error);
                ui.add_space(8.0);
            }
            if theme::soft_button(
                ui,
                &palette,
                None,
                &gettext(locale, "1. Get your Soloist key from Spotify"),
                false,
            )
            .clicked()
            {
                app.actions.push(Action::OpenUrl(
                    "https://developer.spotify.com/dashboard/soloist".into(),
                ));
            }
            ui.label(gettext(locale, "Create a Soloist key in Spotify's developer dashboard. This is separate from your Web API app's Client ID."));
            ui.add_space(8.0);
            ui.label(egui::RichText::new(gettext(locale, "Soloist makes its own Spotify connections. Spotifast's proxy settings do not route its audio traffic.")).small().color(palette.secondary));
            ui.add_space(8.0);
            ui.add(
                egui::TextEdit::singleline(&mut app.soloist_key)
                    .password(true)
                    .desired_width(ui.available_width())
                    .hint_text(gettext(locale, "Paste your personal Soloist key")),
            );
            ui.add_space(8.0);
            let retry = app.settings.soloist_enabled
                && app.soloist_key.is_empty()
                && !matches!(app.soloist_status, Status::NeedsKey);
            let label = if retry {
                gettext(locale, "Retry with saved key")
            } else {
                gettext(locale, "2. Download and enable Soloist")
            };
            ui.add_enabled_ui(retry || !app.soloist_key.trim().is_empty(), |ui| {
                if theme::pill_button(ui, &palette, &label, true).clicked() {
                    app.actions.push(if retry {
                        Action::RetrySoloist
                    } else {
                        Action::SetSoloist(true)
                    });
                }
            });
            if matches!(app.soloist_status, Status::Off)
                && app.soloist_key.trim().is_empty()
                && theme::soft_button(
                    ui,
                    &palette,
                    None,
                    &gettext(locale, "Enable with saved key"),
                    false,
                )
                .clicked()
            {
                app.actions.push(Action::RetrySoloist);
            }
            ui.add_space(8.0);
            ui.label(egui::RichText::new(gettext(locale, "Downloads directly from Spotify and saves your key in the system credential store. By enabling Soloist, you use Spotify's separate software under its terms." )).small().color(palette.secondary));
        }
        Status::Starting => {
            ui.horizontal(|ui| {
                ui.spinner();
                ui.label(gettext(locale, "Starting Soloist…"));
            });
        }
        Status::Downloading { received, total } => {
            ui.label(gettext(locale, "Downloading Soloist from Spotify…"));
            let progress = if *total == 0 {
                0.0
            } else {
                *received as f32 / *total as f32
            };
            ui.add(
                egui::ProgressBar::new(progress)
                    .show_percentage()
                    .animate(true),
            );
        }
        Status::Pairing { name, .. } => {
            ui.label(egui::RichText::new(gettext(locale, "3. Pair once in Spotify")).strong());
            ui.label(gettext(locale, "Open Spotify on the same network, using the account signed in here. In the device picker, choose:"));
            ui.label(egui::RichText::new(name).strong());
            ui.label(gettext(locale, "Spotifast will notice when pairing finishes. Future starts reconnect automatically."));
        }
        Status::Ready {
            name,
            version,
            expires,
        } => {
            let ready = app.soloist_device_id.is_some();
            ui.label(
                egui::RichText::new(if ready {
                    gettext(locale, "Soloist is ready")
                } else {
                    gettext(locale, "Confirming your Spotify account…")
                })
                .strong(),
            );
            ui.label(name);
            if !ready {
                ui.label(gettext(locale, "If this takes a while, make sure Spotify is signed in to the same account as Spotifast, then select this device there."));
            }
            let date = jiff::Timestamp::from_second((*expires).min(i64::MAX as u64) as i64)
                .ok()
                .map(|at| at.to_zoned(jiff::tz::TimeZone::system()).date().to_string())
                .unwrap_or_default();
            ui.label(
                gettext(locale, "Version {version} · expires {date}")
                    .replace("{version}", version)
                    .replace("{date}", &date),
            );
            if app.soloist_update.is_some() {
                ui.label(gettext(locale, "A fresh build is downloaded. It will be applied when Soloist is inactive or Spotifast restarts."));
            } else {
                ui.label(gettext(locale, "Spotifast checks daily for a replacement during the last 30 days of each build. Updates wait until Soloist is inactive."));
            }
            ui.label(gettext(locale, "Spotify chooses playback quality. Soloist does not report the delivered audio format yet."));
        }
    }
    if app.settings.soloist_enabled {
        ui.add_space(10.0);
        if theme::soft_button(
            ui,
            &palette,
            None,
            &gettext(locale, "Turn off Soloist"),
            false,
        )
        .clicked()
        {
            app.actions.push(Action::SetSoloist(false));
        }
    }
    ui.add_space(10.0);
    if theme::soft_button(
        ui,
        &palette,
        None,
        &gettext(locale, "Soloist setup, privacy and limitations"),
        false,
    )
    .clicked()
    {
        app.actions
            .push(Action::OpenUrl("https://spotifast.rocks/soloist/".into()));
    }
    ui.add_space(10.0);
}
