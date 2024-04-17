# powerdialog

Dialogue of shutdown, sleep, reboot PC

https://github.com/govnorice/powerdialog/assets/80642969/03da97c3-927d-4c93-ae7d-6adda75fc99b

## Warning!
### In order for the application to use commands, it is recommended to register it in **/etc/sudoers**

/etc/sudoers
```/etc/sudoers
root ALL=(ALL) NOPASSWD: /path_to_the_program
```
## Wayland tailing WM warning

GTK3 cannot make window float on Wayland using GDK_WINDOW_TYPE_HINT_DIALOG!

**Use floating rules for your wayland WM!**

### Example hypland:

hyprland.conf

```hyprland.conf
windowrule = float, ^(mixerdialog)$
```
