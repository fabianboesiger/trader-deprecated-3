const DELTA = 0.000001;

function updateField(elem, value, colored = false) {
    let prev = parseFloat(elem.innerText);
    if (value !== null) {
        let curr = parseFloat(value.toFixed(4));
        if (colored) {
            if (curr > prev) {
                elem.classList.remove("lower");
                elem.classList.add("higher");
            } else
            if (curr < prev) {
                elem.classList.remove("higher");
                elem.classList.add("lower");
            }/* else {
                elem.classList.remove("green");
                elem.classList.remove("red");
            }*/
        }
        elem.innerText = "" + curr;
    } else {
        elem.innerText = "-";
    }
}

document.addEventListener("DOMContentLoaded", event => {
    let assetsTable = document.getElementById("assets").getElementsByTagName("tbody")[0];
    let tradesTable = document.getElementById("trades").getElementsByTagName("tbody")[0];

    let equityChart = new Chart(document.getElementById("chart-equity"), {
        type: 'line',
        data: {
            labels: [],
            datasets: [{ 
                data: [],
                label: "Equity",
                borderColor: "#06d",
                backgroundColor: "#222",
                fill: true
            }]
        },
        options: {
            legend: {
                labels: {
                    fontColor: "#ddd",
                    fontFamily: "Fira Code"
                }
            },
            scales: {
                xAxes: [{
                    ticks: {
                        fontColor: "#ddd",
                        fontFamily: "Fira Code"
                    }
                }],
                yAxes: [{
                    ticks: {
                        fontColor: "#ddd",
                        fontFamily: "Fira Code",
                        callback: function(value, index, values) {
                            return value + " USDT";
                        }
                    }
                }]
            }
        }
    });

    let assetProfitsChart = new Chart(document.getElementById("chart-asset-profits"), {
        type: 'bar',
        data: {
            labels: [],
            datasets: [{ 
                data: [],
                label: "Average Rate of Profit per Trade",
                backgroundColor: "#06d",
            }]
        },
        options: {
            legend: {
                labels: {
                    fontColor: "#ddd",
                    fontFamily: "Fira Code"
                }
            },
            scales: {
                xAxes: [{
                    ticks: {
                        fontColor: "#ddd",
                        fontFamily: "Fira Code"
                    }
                }],
                yAxes: [{
                    ticks: {
                        fontColor: "#ddd",
                        fontFamily: "Fira Code",
                        callback: function(value, index, values) {
                            return value + "%";
                        }
                    }
                }]
            }
        }
    });

    let profitPerTradeChart = new Chart(document.getElementById("chart-profit-per-trade"), {
        type: 'line',
        data: {
            labels: [],
            datasets: [{ 
                data: [],
                label: "Rate of Profit per Trade",
                borderColor: "#06d",
                backgroundColor: "#222",
                fill: true
            }]
        },
        options: {
            legend: {
                labels: {
                    fontColor: "#ddd",
                    fontFamily: "Fira Code"
                }
            },
            scales: {
                xAxes: [{
                    ticks: {
                        fontColor: "#ddd",
                        fontFamily: "Fira Code"
                    }
                }],
                yAxes: [{
                    ticks: {
                        fontColor: "#ddd",
                        fontFamily: "Fira Code",
                        callback: function(value, index, values) {
                            return value + "%";
                        }
                    }
                }]
            }
        }
    });
    
    let winLossChart = new Chart(document.getElementById("chart-win-loss"), {
        type: 'doughnut',
        data: {
            labels: ["Total Wins", "Total Losses"],
            datasets: [{
                label: "Total",
                data: [0, 0],
                borderColor: ["#6d0", "#d06"],
                backgroundColor: ["#6d0", "#d06"],
            }]
        },
        options: {
            legend: {
                labels: {
                    fontColor: "#ddd",
                    fontFamily: "Fira Code"
                }
            },
        }
    });

    let winLossProfitsChart = new Chart(document.getElementById("chart-win-loss-profits"), {
        type: 'doughnut',
        data: {
            labels: ["Wins USDT", "Losses USDT"],
            datasets: [{
                label: "Total",
                data: [0, 0],
                borderColor: ["#6d0", "#d06"],
                backgroundColor: ["#6d0", "#d06"],
            }]
        },
        options: {
            legend: {
                labels: {
                    fontColor: "#ddd",
                    fontFamily: "Fira Code"
                }
            },
        }
    });

    let rateOfProfitElem = document.getElementById("rate-of-profit");
    let lastDayRateOfProfitElem = document.getElementById("last-day-rate-of-profit");

    let total = 1000.0;
    let liveTotal = null;
    let assets = {};
    let trades = {};

    
    addAsset("USDT");
    updateQuantity("USDT", 1000.0);
    updateValue("USDT", 1.0);
    
    function addTrade(trade) {
        if (trade.sell.asset === "USDT") {
            let asset = trade.buy.asset;

            if (!(asset in trades)) {
                trades[asset] = [];
            }

            trades[asset].push({
                buy: {
                    got: parseFloat(trade.buy.quantity),
                    for: parseFloat(trade.sell.quantity),
                    timestamp: new Date(trade.timestamp)
                },
                sell: null,
            });

            let tradeStore = trades[asset][trades[asset].length - 1];

            let rowElem = document.createElement("tr");
            rowElem.id = asset + "-" + (trades[asset].length - 1);

            let timestampElem = document.createElement("td");
            timestampElem.classList.add("timestamp");
            timestampElem.innerText += tradeStore.buy.timestamp.toLocaleString();
            rowElem.appendChild(timestampElem);

            let assetElem = document.createElement("td");
            assetElem.classList.add("asset");
            assetElem.innerText = asset;
            rowElem.appendChild(assetElem);

            let quantityElem = document.createElement("td");
            quantityElem.classList.add("quantity");
            quantityElem.innerText = tradeStore.buy.got.toFixed(4);
            rowElem.appendChild(quantityElem);
            
            let usdtElem = document.createElement("td");
            usdtElem.classList.add("usdt");
            usdtElem.innerHTML = "-";
            rowElem.appendChild(usdtElem);

            tradesTable.insertBefore(rowElem, tradesTable.firstChild);
        } else {
            let asset = trade.sell.asset;

            let tradeStore = trades[asset][trades[asset].length - 1];
            tradeStore.sell = {
                got: parseFloat(trade.buy.quantity),
                for: parseFloat(trade.sell.quantity),
                timestamp: new Date(trade.timestamp)
            }

            let rowElem = document.getElementById(asset + "-" + (trades[asset].length - 1));

            //let duration = (tradeStore.sell.timestamp.getTime() - tradeStore.buy.timestamp.getTime()) / 1000.0 / 60.0;

            let timestampElem = rowElem.getElementsByClassName("timestamp")[0];
            //durationElem.innerText = (tradeStore.sell.timestamp.getTime() - tradeStore.buy.timestamp.getTime()) / 1000 / 60;
            timestampElem.innerText += " - " + tradeStore.sell.timestamp.toLocaleString();
            

            let usdtElem = rowElem.getElementsByClassName("usdt")[0];
            let profits = (tradeStore.sell.got - tradeStore.buy.for);
            usdtElem.innerText = profits.toFixed(4);
            if (profits > 0) {
                usdtElem.classList.add("green");
            } else

            if (profits < 0) {
                usdtElem.classList.add("red");
            }
            
            total += profits;
            
            equityChart.data.labels.push(tradeStore.buy.timestamp.toLocaleDateString());
            equityChart.data.datasets[0].data.push(total);
            equityChart.update();
            
            profitPerTradeChart.data.labels.push(tradeStore.buy.timestamp.toLocaleDateString());
            profitPerTradeChart.data.datasets[0].data.push(profits / tradeStore.sell.got * 100.0);
            profitPerTradeChart.update();
            
            assetProfitsChart.data.labels = [];
            assetProfitsChart.data.datasets[0].data = [];
            let wins = 0;
            let losses = 0;
            let winsUsdt = 0.0;
            let lossesUsdt = 0.0;
            let data = [];
            let firstTrade = null;
            let yesterday = new Date();
            yesterday.setTime(Date.now());
            yesterday.setDate(yesterday.getDate() - 1);

            let lastDayProfit = 0.0;
            let lastDayTrades = 0;
            let totalProfits = 0.0;
            let totalTrades = 0;

            for (let [asset, assetTrades] of Object.entries(trades)) {
                // Find profits per asset.
                let totalProfitsPerAsset = 0.0;
                let count = 0;
                for (let assetTrade of assetTrades) {

                    if (firstTrade === null || assetTrade.buy.timestamp < firstTrade) {
                        firstTrade = assetTrade.buy.timestamp;
                    }

                    if (assetTrade.sell !== null) {

                        if (firstTrade === null || assetTrade.sell.timestamp < firstTrade) {
                            firstTrade = assetTrade.sell.timestamp;
                        }

                        let profits = (assetTrade.sell.got - assetTrade.buy.for);   
                        if (profits > 0) {
                            wins++;
                            winsUsdt += profits;
                        } else
                        if (profits < 0) {
                            losses++;
                            lossesUsdt -= profits;
                        }
                        totalProfitsPerAsset += profits / assetTrade.sell.got;
                        count++;
                        
                        if (assetTrade.sell.timestamp > yesterday) {
                            lastDayProfit += profits / assetTrade.sell.got;
                            lastDayTrades++;
                        } 
                    }
                }

                totalProfits += totalProfitsPerAsset;
                totalTrades += count;

                if (count > 0) {
                    data.push({
                        profit: totalProfitsPerAsset / count * 100,
                        asset: asset
                    });
                }
            }

            data.sort((a, b) => b.profit - a.profit);
            for (let e of data) {
                assetProfitsChart.data.labels.push(e.asset);
                assetProfitsChart.data.datasets[0].data.push(e.profit);
            }
            assetProfitsChart.update();

            winLossChart.data.datasets[0].data = [wins, losses];
            winLossChart.update();
            winLossProfitsChart.data.datasets[0].data = [winsUsdt, lossesUsdt];
            winLossProfitsChart.update();

            rateOfProfitElem.innerText = (totalProfits / totalTrades * 100.0).toFixed(2);
            lastDayRateOfProfitElem.innerText = (lastDayProfit / lastDayTrades * 100.0).toFixed(2);
        }


    }
    
    function addAsset(asset) {
        if (!(asset in assets)) {
            assets[asset] = {
                quantity: 0.0,
                value: null,
                usdt: null,
            };

            let rowElem = document.createElement("tr");
            rowElem.id = asset;
            rowElem.style.display = "none";

            let assetElem = document.createElement("td");
            assetElem.classList.add("asset");
            assetElem.innerText = asset;
            rowElem.appendChild(assetElem);

            let quantityElem = document.createElement("td");
            quantityElem.classList.add("quantity");
            rowElem.appendChild(quantityElem);
            /*
            let valueElem = document.createElement("td");
            valueElem.classList.add("value");
            rowElem.appendChild(valueElem);
            */
            let usdtElem = document.createElement("td");
            usdtElem.classList.add("usdt");
            rowElem.appendChild(usdtElem);

            assetsTable.appendChild(rowElem);
        }
        
    }
    
    function updateQuantity(asset, quantity) {
        assets[asset].quantity += parseFloat(quantity);

        let rowElem = document.getElementById(asset);
        updateField(rowElem.getElementsByClassName("quantity")[0], assets[asset].quantity);

        if (Math.abs(assets[asset].quantity) < DELTA) {
            rowElem.style.display = "none";
        } else {
            rowElem.style.display = "table-row";
        }

        updateUsdt(asset);
    }
    
    function updateValue(asset, value) {
        assets[asset].value = parseFloat(value);

        /*
        let rowElem = document.getElementById(asset);
        updateField(rowElem.getElementsByClassName("value")[0], assets[asset].value, true);
        */

        updateUsdt(asset);
    }

    function updateUsdt(asset) {
        if (assets[asset].value !== null) {
            assets[asset].usdt = assets[asset].quantity * assets[asset].value;
        }

        let rowElem = document.getElementById(asset);
        updateField(rowElem.getElementsByClassName("usdt")[0], assets[asset].usdt, true);

        updateTotal();
    }
    
    function updateTotal() {
        liveTotal = 0.0;
        let setNull = false;
        for (let asset of Object.values(assets)) {
            if (asset.quantity > DELTA) {
                if (asset.usdt === null) {
                    setNull = true;
                } else {
                    liveTotal += asset.usdt;
                }
            }
            
        }
        if (setNull) {
            liveTotal = null;
        }

        let totalElem = document.getElementById("total");
        updateField(totalElem, liveTotal, true);
    }
    
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