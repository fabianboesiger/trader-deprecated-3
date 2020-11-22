document.addEventListener("DOMContentLoaded", event => {
    let main = document.getElementsByTagName("main");

    let uri = "ws://" + location.host + "/socket";
    let ws = new WebSocket(uri);

    ws.onopen = function() {}

    ws.onmessage = function(message) {
	console.log(message);
        main.innerHTML = message.data;
    };
});
