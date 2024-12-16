import { invoke } from "@tauri-apps/api/core";

function handleError(err: string) {
    let el = document.querySelector("#errors");
    if (el) {
        el.innerHTML += `<p>${err}</p>`;
    }
}

window.addEventListener("DOMContentLoaded", () => {
    document.querySelector("#panics")?.addEventListener("click", () => {
        invoke("panics");
    });

    document.querySelector("#panics-async")?.addEventListener("click", () => {
        let promise = invoke("panics_async");

        setInterval(() => {
            console.log(promise);
        }, 1000);
    });

    document
        .querySelector("#error-as-string")
        ?.addEventListener("click", () => {
            invoke("error_as_string").catch(handleError);
        });

    document
        .querySelector("#thiserror")
        ?.addEventListener("click", () => {
            invoke("using_thiserror").catch(handleError);
        });

    document
        .querySelector("#anyhow")
        ?.addEventListener("click", () => {
            invoke("using_thiserror_and_anyhow").catch(handleError);
        });
});
