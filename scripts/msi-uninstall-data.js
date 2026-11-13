// Runs during MSI uninstall only (see build-msi.sh): asks whether to delete
// local app data, then removes it. Prefer the still-installed exe's
// --uninstall-wipe when present; otherwise delete the data folder.
// Placeholders __APP_ID__, __APP_LABEL__, __BIN_NAME__ are replaced by build-msi.sh.
// The whole body is guarded: any failure here must never block uninstall.
function PromptDeleteAppData() {
    try {
        var shell = new ActiveXObject("WScript.Shell");
        var fso = new ActiveXObject("Scripting.FileSystemObject");
        try {
            var desk = shell.SpecialFolders("Desktop") + "\\__APP_LABEL__.lnk";
            if (fso.FileExists(desk)) {
                fso.DeleteFile(desk, true);
            }
            var pin =
                shell.ExpandEnvironmentStrings("%APPDATA%") +
                "\\Microsoft\\Internet Explorer\\Quick Launch\\User Pinned\\TaskBar\\__APP_LABEL__.lnk";
            if (fso.FileExists(pin)) {
                fso.DeleteFile(pin, true);
            }
        } catch (ePin) {}
        var dir = shell.ExpandEnvironmentStrings("%APPDATA%") + "\\__APP_ID__";
        if (!fso.FolderExists(dir)) {
            return;
        }
        // Button flags 4 = Yes/No, icon 32 = question, 4096 = system-modal.
        // 60s timeout returns -1 (treated as No) so silent uninstall never hangs.
        var flags = 4 + 32 + 4096;
        var del = shell.Popup(
            "Delete __APP_LABEL__ local data?\n\n" +
                dir +
                "\n\nYes - delete forever.\nNo - keep data.",
            60,
            "Uninstall __APP_LABEL__",
            flags
        ) == 6;
        if (!del) {
            return;
        }
        try {
            var install = Session.Property("INSTALLFOLDER");
            if (install && install.charAt(install.length - 1) !== "\\") {
                install += "\\";
            }
            var exe = install ? install + "__BIN_NAME__.exe" : "";
            if (exe && fso.FileExists(exe)) {
                shell.Run('"' + exe + '" --uninstall-wipe all', 0, true);
            }
        } catch (eWipe) {}
        if (fso.FolderExists(dir)) {
            fso.DeleteFolder(dir, true);
        }
    } catch (e) {
        // ignore — uninstall must continue
    }
}
