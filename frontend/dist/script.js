"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
function getAvailability(location) {
    return __awaiter(this, void 0, void 0, function* () {
        return new Promise((resolve) => setTimeout(() => {
            resolve({
                current: 3,
                occupiedSince: "2025-11-16T17:05:20Z",
                lastUpdateTime: "2025-11-16T19:25:10Z",
                history: {
                    monday: [
                        0.9, 0.2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0,
                    ],
                    tuesday: [
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0.8, 1.9, 2.1, 2.1,
                        2.1, 2.8, 2.1, 1.4,
                    ],
                    wednesday: [
                        0.8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0,
                    ],
                    thursday: [
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1.3, 2.6, 2.5,
                        2.5, 2, 1.5,
                    ],
                    friday: [
                        0.7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1.2, 1.7, 1.7, 1.7, 2.2,
                        2.5, 3, 4, 4, 3.1, 2.1,
                    ],
                    saturday: [
                        1.2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0.7, 1.2, 1.9, 2.1, 2.2,
                        2.3, 2.4, 2.8, 3.2, 3.4, 2.9,
                    ],
                    sunday: [
                        2.6, 2.4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0.6, 1.2, 1.8, 2.1,
                        2.4, 2.5, 2.5, 2.0, 1.4,
                    ],
                },
            });
        }, 300));
    });
}
function loadData(data, table) {
    if (!table)
        return console.error("Missing table for data");
    const statusBlock = table.parentElement;
    if (!statusBlock || !statusBlock.classList.contains("status"))
        return console.error("Table not part of a status block!");
    let occupiedSince = data.occupiedSince ? new Date(data.occupiedSince) : null;
    let currentTime = new Date(data.lastUpdateTime);
    let status = "closed";
    if (data.current > 0 &&
        occupiedSince &&
        currentTime.getTime() - occupiedSince.getTime() > 15 * 60 * 1000)
        status = "open";
    statusBlock.className = "status " + status;
    let lastUpdate = statusBlock.querySelector(".last-update");
    if (lastUpdate)
        lastUpdate.textContent = currentTime.toLocaleString();
    const hourStart = 5;
    let occupancy = [
        data.history.sunday,
        data.history.monday,
        data.history.tuesday,
        data.history.wednesday,
        data.history.thursday,
        data.history.friday,
        data.history.saturday,
    ];
    for (let i = 0; i < 7; i++) {
        const earlyHours = occupancy[(i + 1) % 7].splice(0, hourStart);
        occupancy[i].push(...earlyHours);
    }
    let startDate = currentTime.getDay();
    let currentHour = currentTime.getHours() - hourStart;
    if (currentHour < 0) {
        currentHour += 24;
        startDate = (startDate + 6) % 7;
    }
    const weekdays = [
        "Sonntag",
        "Montag",
        "Dienstag",
        "Mittwoch",
        "Donnerstag",
        "Freitag",
        "Samstag",
    ];
    const recordMaxOccupancy = Math.max(data.current, ...occupancy.map((o) => Math.max(...o)));
    for (const row of table.querySelectorAll("tr[data-days]")) {
        let weekday = (startDate + parseInt(row.getAttribute("data-days") || "") + 7) % 7;
        let tds = row.querySelectorAll("td");
        tds[0].textContent = weekdays[weekday];
        for (let h = 0; h < 24; h++) {
            let v = Math.round((occupancy[weekday][h] / recordMaxOccupancy) * 10);
            if (weekday == startDate && h == currentHour) {
                v = Math.round((data.current / recordMaxOccupancy) * 10);
                tds[1 + h].setAttribute("a", v.toString());
                tds[1 + h].setAttribute("v", v.toString());
                tds[1 + h].setAttribute("data-raw", data.current.toString());
            }
            else {
                tds[1 + h].setAttribute("v", v.toString());
                tds[1 + h].removeAttribute("a");
            }
        }
    }
}
getAvailability("Utopia").then((d) => loadData(d, document.querySelector(".utopia table.date-stats")));
getAvailability("Bunker").then((d) => loadData(d, document.querySelector(".bunker table.date-stats")));
