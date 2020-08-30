document.addEventListener("DOMContentLoaded", event => {
    let uri = "ws://" + location.host + "/socket";
    let ws = new WebSocket(uri);

    ws.onopen = function() {
        console.log("connected");
    }

    ws.onmessage = function(message) {
        let data = JSON.parse(message.data);
        console.log(data);
    };
});