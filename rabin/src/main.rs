use clap::{AppSettings, clap_app};

fn main() {
    let matches = clap_app!(rabin => 
                    (setting: AppSettings::ColoredHelp)
                    (version: "0.1")
                    (about: "L*de lag gaye")
                    (@arg verbose: -v --verbose "Be more verbose (Aur jyada gaali)")
                    (@subcommand phone => 
                        (setting: AppSettings::ColoredHelp)
                        (about: "Call karo saale ko !")
                        (@arg modules: "(Optional Arg) List modules")
                     )
                    (@subcommand library => 
                        (setting: AppSettings::ColoredHelp)
                        (about: "Library ki book to gayi")
                        (@arg user: +required "Username of the new user")
                     )
                    (@subcommand vasooli => 
                        (setting: AppSettings::ColoredHelp)
                        (about: "Paise de de Rabin !")
                        (@arg user: +required "Username of user")
                        (@arg module: +required "Name of pre-available module")
                     )
                    (@subcommand safalta => 
                        (setting: AppSettings::ColoredHelp)
                        (about: "Thoda aur safal banaya jaye")
                        (@arg user: +required "Username of user")
                        (@arg module: +required "Name of pre-available module")
                     )
                    (@subcommand crush =>
                        (setting: AppSettings::ColoredHelp)
                        (about: "Oooooo... \u{1F60F}\u{1F60F}... Rabin ko crush dilwaye")
                    )).get_matches();
}
