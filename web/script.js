document.addEventListener("DOMContentLoaded", event => {
    let rateOfProfitElem = document.getElementById("rate-of-profit");
    let lastDayRateOfProfitElem = document.getElementById("last-day-rate-of-profit");
    let estimatedProfitsPerDayElem = document.getElementById("estimated-profits");

    let uri = "ws://" + location.host + "/socket";
    let ws = new WebSocket(uri);

    ws.onopen = function() {}

    ws.onmessage = function(message) {
        let data = JSON.parse(message.data);
        if ("Trade" in data) {
            let trade = data.Trade;
            addTrade(trade);
            addAsset(trade.buy.asset);
            addAsset(trade.sell.asset);
            updateQuantity(trade.buy.asset, trade.buy.quantity);
            updateQuantity(trade.sell.asset, -trade.sell.quantity);
        } else
        if ("Value" in data) {
            let value = data.Value;
            addAsset(value.market.base);
            updateValue(value.market.base, value.value);
        }
    };
});