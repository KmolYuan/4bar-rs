import * as wasm from "./pkg/four_bar_ui.js";

// Module level references
const reader = new FileReader();
const a = document.createElement("a");
const input = document.createElement("input");
input.type = "file";

// Utility functions
window.save_file = (s, file_name) => {
    a.download = file_name;
    a.href = window.URL.createObjectURL(new Blob([s], {type: "application/octet-stream"}));
    a.click();
};
window.open_file = (format, done, cancel) => {
    input.onclick = () => {
        document.body.onfocus = () => {
            if (input.files.length > 0)
                reader.readAsText(input.files[0]);
            else
                cancel();
            document.body.onfocus = null;
        };
    };
    reader.onload = () => done(input.files[0].name, reader.result);
    input.accept = format;
    input.click();
};
window.get_host = () => location.href;
window.get_username = () => ("; " + document.cookie).split("; username=").pop().split(";").shift();
window.login = (account, body, done) =>
    fetch(location.href + "login/" + account, {
        method: "POST",
        body: body,
        headers: {"content-type": "application/json"},
        mode: "cors",
    }).then(res => done(res.ok));
window.logout = done =>
    fetch(location.href + "logout", {
        method: "POST",
        mode: "cors",
    }).then(res => done(res.ok));

// Startup WebAssembly
wasm.default().then(() => wasm.start("main_canvas"));
