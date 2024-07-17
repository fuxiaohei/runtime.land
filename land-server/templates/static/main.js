document.addEventListener("DOMContentLoaded", function () {

    // timeago change to human readable time
    (() => {
        let fn = function () {
            document.querySelectorAll(".time-ago").forEach((el) => {
                let timestamp = parseInt(el.getAttribute("data-x-timeago")) * 1000;
                let dt = new Date(timestamp);
                el.innerText = timeago.format(dt, "en_US");
            });
        };
        fn();
        setInterval(fn, 1000 * 30);
    })();

    // htmx response error handle
    (() => {
        document.body.addEventListener('htmx:responseError', function (evt) {
            let message = `<div class="err-message">${evt.detail.error}/<div>`;
            evt.detail.target.innerHTML = message;
            evt.detail.target.classList.add("htmx-settling");
            setTimeout(() => {
                evt.detail.target.classList.remove("htmx-settling");
            }, 2000);
        });
    })()

})