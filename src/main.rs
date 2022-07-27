use chrono::{Duration, Local, Timelike};
use std::{
    process::{self, Command},
    thread,
};

fn get_current_time() -> (u32, u32) {
    let now = Local::now();
    return (now.hour(), now.minute());
}

fn set_wallpaper(hour: u32) {
    /* 6AM morning
     * 10 AM day
     * 6 PM evening
     * 10 PM night
     */

    if hour < 6 {
        // night
        println!("setting night");
        Command::new("gsettings")
            .arg("set")
            .arg("org.gnome.desktop.background")
            .arg("picture-uri-dark")
            .arg("file:////home/cyn/Dropbox/Wallpapers/daytime/tropic_island_night.jpg");
    } else if hour >= 6 && hour < 10 {
        // morning
        Command::new("gsettings")
            .arg("set")
            .arg("org.gnome.desktop.background")
            .arg("picture-uri-dark")
            .arg("file:////home/cyn/Dropbox/Wallpapers/daytime/tropic_island_morning.jpg");
    } else if hour >= 10 && hour < 18 {
        // day
        Command::new("gsettings")
            .arg("set")
            .arg("org.gnome.desktop.background")
            .arg("picture-uri-dark")
            .arg("file:////home/cyn/Dropbox/Wallpapers/daytime/tropic_island_day.jpg");
    } else if hour >= 18 && hour < 22 {
        // evening
        Command::new("gsettings")
            .arg("set")
            .arg("org.gnome.desktop.background")
            .arg("picture-uri-dark")
            .arg("file:////home/cyn/Dropbox/Wallpapers/daytime/tropic_island_evening.jpg");
    } else {
        // night
        Command::new("gsettings")
            .arg("set")
            .arg("org.gnome.desktop.background")
            .arg("picture-uri-dark")
            .arg("file:////home/cyn/Dropbox/Wallpapers/daytime/tropic_island_night.jpg");
    }
}

fn main() {
    loop {
        let sleep_hours;
        let mut sleep_minutes = 0;
        let (current_hour, current_minute) = get_current_time();

        if current_hour < 6 {
            if current_minute != 0 {
                sleep_hours = 6 - current_hour - 1;
                sleep_minutes = 60 - current_minute;
            } else {
                sleep_hours = 6 - current_hour;
            }
        } else if current_hour < 10 {
            if current_minute != 0 {
                sleep_hours = 10 - current_hour - 1;
                sleep_minutes = 60 - current_minute;
            } else {
                sleep_hours = 10 - current_hour;
            }
        } else if current_hour < 18 {
            if current_minute != 0 {
                sleep_hours = 18 - current_hour - 1;
                sleep_minutes = 60 - current_minute;
            } else {
                sleep_hours = 18 - current_hour;
            }
        } else {
            if current_minute != 0 {
                sleep_hours = 24 - current_hour - 1;
                sleep_minutes = 60 - current_minute;
            } else {
                sleep_hours = 24 - current_hour;
            }
        }
        set_wallpaper(current_hour);
        match (Duration::hours(sleep_hours.into()) + Duration::minutes(sleep_minutes.into()))
            .to_std()
        {
            Ok(d) => thread::sleep(d),
            Err(..) => process::exit(1),
        }
    }
}
