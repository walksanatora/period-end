use which::which;
use std::process::Command;

pub fn get_upgradable_packages() -> usize {
    if which("apt").is_ok(){
        let updates = Command::new("apt")
        .args(["list","--upgradable"])
        .output().unwrap();
        if updates.status.code().unwrap_or(1) == 0 {
            let tmp = &updates.stdout.into_boxed_slice();
            let o = String::from_utf8_lossy(tmp);
            o.lines().count() - 1 // because apt includes a "listing ... done" at the end
        } else {
            #[cfg(debug_assertions)]
            eprintln!("! apt list failed");
            0
        }
    } else if which("paru").is_ok() {
        let updates = Command::new("paru")
        .args(["-Qu"])
        .output().unwrap();
        if updates.status.code().unwrap_or(1) == 0 {
            let tmp = &updates.stdout.into_boxed_slice();
            let o = String::from_utf8_lossy(tmp);
            o.lines().count()
        } else {
            #[cfg(debug_assertions)]
            {
                eprintln!("! paru -Qu failed");
                let slice = updates.stdout.into_boxed_slice();
                eprintln!("{}",String::from_utf8_lossy(&slice));
            }
            0
        }
    } else if which("pacman").is_ok() {
        let updates = Command::new("pacman")
        .args(["-Qu"])
        .output().unwrap();
        if updates.status.code().unwrap_or(1) == 0 {
            let tmp = &updates.stdout.into_boxed_slice();
            let o = String::from_utf8_lossy(tmp);
            o.lines().count()
        } else {
            #[cfg(debug_assertions)]
            eprintln!("! pacman -Qu failed");
            0
        }
    } else {0}
}