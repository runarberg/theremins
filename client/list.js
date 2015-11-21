var ws = new WebSocket("ws://192.168.2.101:8001/list");

ws.onopen = list;

function list() {
  "use strict";

  var roomsUl = byId("rooms");

  ws.onmessage = render;

  function render(msg) {
    var data = JSON.parse(msg.data);
    var rooms = data.reduce(function(acc, room) {
      if (room === "/list") {
        // Discard data from this room as it is irrelevant.
        return acc;
      }

      if (typeof acc[room] !== "number") {
        acc[room] = 1;
      } else {
        acc[room] += 1;
      }
      return acc;
    }, {});

    [].forEach.call(roomsUl.children, function(li, i) {
      var roomNames = Object.keys(rooms);
      if (i < roomNames.length) {
        // Update.
        var room = roomNames[i];
        var count = rooms[room];

        var roomAnchor = li.querySelector("a");
        roomAnchor.href = room;
        roomAnchor.textContent = unescape(room);

        var countNode = li.querySelector(".count");
        countNode.textContent = "(" + count + ")";
      } else {
        // Exit.
        roomsUl.removeChild(li);
      }
    });

    Object.keys(rooms).slice(roomsUl.children.length).forEach(function(room) {
      // Enter.
      var li = document.createElement("li");
      var count = rooms[room];

      var roomAnchor = document.createElement("a");
      roomAnchor.href = room;
      roomAnchor.textContent = unescape(room);

      var nameNode = document.createElement("span");
      nameNode.className = "name";
      nameNode.appendChild(roomAnchor);

      var countNode = document.createElement("span");
      countNode.className = "count";
      countNode.textContent = "(" + count + ")";

      li.appendChild(nameNode);
      li.appendChild(document.createTextNode(" "));
      li.appendChild(countNode);
      roomsUl.appendChild(li);
    })
  }
}

function byId(id) { return document.getElementById(id); }
