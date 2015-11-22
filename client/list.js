var ws = new WebSocket("ws://192.168.2.101:8001/list");

ws.onopen = list;

function list() {
  "use strict";

  var roomsUl = byId("rooms");

  ws.onmessage = render;

  function render(msg) {
    var data = JSON.parse(msg.data);
    var rooms = data.reduce(function(acc, roomName) {
      if (roomName === "/list") {
        // Discard data from this room as it is irrelevant.
        return acc;
      }

      var room = acc.find(function(d) { return d.name === roomName; });
      if (room && typeof room.count === "number") {
        room.count += 1;
      } else {
        acc.push({name: roomName, count: 1});
      }
      return acc;
    }, []);

    rooms.sort(function(a, b) { return b.count - a.count });

    [].forEach.call(roomsUl.children, function(li, i) {
      if (i < rooms.length) {

        // Update.
        var room = rooms[i];

        var roomAnchor = li.querySelector("a");
        roomAnchor.href = room.name;
        roomAnchor.textContent = unescape(room.name);

        var countNode = li.querySelector(".count");
        countNode.textContent = room.count;
      } else {

        // Exit.
        roomsUl.removeChild(li);
      }
    });

    rooms.slice(roomsUl.children.length).forEach(function(room) {

      // Enter.
      var li = document.createElement("li");

      var roomAnchor = document.createElement("a");
      roomAnchor.href = room.name;
      roomAnchor.textContent = unescape(room.name);

      var nameNode = document.createElement("span");
      nameNode.className = "name";
      nameNode.appendChild(roomAnchor);

      var countNode = document.createElement("span");
      countNode.className = "count";
      countNode.textContent = room.count;

      li.appendChild(nameNode);
      li.appendChild(document.createTextNode(" — "));
      li.appendChild(countNode);
      roomsUl.appendChild(li);
    })
  }
}

function byId(id) { return document.getElementById(id); }
