# powerdialog

Dialogue of shutdown, logout, sleep, reboot PC

https://github.com/govnorice/powerdialog/assets/80642969/c9da78da-2c6e-497f-b96e-89570ffad2ca

# INSTALL ARCH LINUX AND OTHER

## I USE ARCH LINUX BTW?
Just install the package from the AUR, damn it!
```
yay -S powerdialog
```
# OR
1. Download executable file and move to /usr/bin:

```
cd download_location
chmod +x powerdialog
sudo mv powerdialog /usr/bin/powerdialog
```
2. Create polkit rule in "/etc/polkit-1/rules.d/powerdialog.rules":
```
polkit.addRule(function(action, subject) {
    if (action.id == "org.freedesktop.login1.power-off" ||
        action.id == "org.freedesktop.login1.reboot" ||
        action.id == "org.freedesktop.login1.suspend" ||
        action.id == "org.freedesktop.login1.hibernate") {
        if (subject.user == "root" &&
            subject.process.binary == "/usr/bin/powerdialog") {
            return polkit.Result.YES;
        }
    }
});
```
3. Restart polkit service
```
sudo systemctl restart polkit.service
```
# HOW TO USE

hyprland example:
```
bind = $mainMod, P, exec, powerdialog
```

