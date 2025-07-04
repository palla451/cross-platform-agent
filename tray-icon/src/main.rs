use gtk::{Menu, MenuItem};
use appindicator3::{Indicator, IndicatorCategory, IndicatorStatus};

// Importa tutti i trait necessari
use gtk::prelude::{MenuShellExt, GtkMenuItemExt, WidgetExt};
use appindicator3::prelude::AppIndicatorExt;

fn main() {
    // Inizializza GTK
    gtk::init().expect("Failed to initialize GTK.");

    // Crea l'indicatore senza icona di default (verrà impostata manualmente)
    let indicator = Indicator::new(
        "agent-tray",
        "",
        IndicatorCategory::ApplicationStatus,
    );

    // Imposta il path dove cercare le icone personalizzate nel container
    indicator.set_icon_theme_path("/app/icons");

    // Imposta l’icona personalizzata (senza estensione .png)
    indicator.set_icon_full("icon-cyber-green", "Cyber Icon Green");

    // Imposta lo stato su attivo
    indicator.set_status(IndicatorStatus::Active);

    // Crea un menu GTK
    let menu = Menu::new();

    // Aggiunge una voce "Esci"
    let quit = MenuItem::with_label("Esci");
    quit.connect_activate(|_| {
        gtk::main_quit();
    });

    menu.append(&quit);
    menu.show_all();

    // Assegna il menu all’indicatore
    indicator.set_menu(Some(&menu));

    // Avvia il loop GTK
    gtk::main();
}
