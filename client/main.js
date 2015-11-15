var τ = 2 * Math.PI;
var twelve_root_2 = Math.pow(2, 1/12);

var ws = new WebSocket("ws://192.168.2.101:8001");
ws.onopen = app;

function app() {
  "use strict";
  var canvas = byId("canvas");
  var controls = byId("controls");
  var minFrequency = controls.querySelector("input[name='min-frequency']");
  var maxFrequency = controls.querySelector("input[name='max-frequency']");

  window.addEventListener("resize", function() {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight - controls.clientHeight;
  });

  var pointers = new Map();
  var sineWaves = [];

  ws.onmessage = function(msg) {
    sineWaves = flatten(JSON.parse(msg.data));
  };

  var x = function(t) {
    var log2_a = Math.log2(+minFrequency.value);
    var log2_a_m_b = Math.log2(+maxFrequency.value) - log2_a;
    return !isNaN(log2_a) && !isNaN(log2_a_m_b) ?
      Math.pow(2, log2_a + t * log2_a_m_b) :
      0;
  };

  var x_inv = function(t) {
    var log2_a = Math.log2(+minFrequency.value);
    var log2_a_m_b = Math.log2(+maxFrequency.value) - log2_a;
    return  !isNaN(log2_a) && !isNaN(log2_a_m_b) ?
      (Math.log2(t) - log2_a) / log2_a_m_b :
      0;
  };
  var y = function(t) { return 1 - t; };
  var y_inv = y;

  var color = "rgb(" + [0, 0, 0].map(function() {
    return (Math.random() * 255)|0;
  }).join(",") + ")";

  canvas.addEventListener("mousedown", function(e) {
    pointers.set("mouse_0", {
      x: e.clientX / canvas.width,
      y: e.clientY / canvas.height
    });
  });

  canvas.addEventListener("mouseup", function() {
    pointers.delete("mouse_0");
  });

  canvas.addEventListener("mousemove", function(e) {
    var mouse = pointers.get("mouse_0");
    if (mouse) {
      mouse.x = e.clientX / canvas.width;
      mouse.y = e.clientY / canvas.height;
    }
  });

  canvas.addEventListener("touchstart", function(e) {
    e.preventDefault();
    forEach(e.changedTouches, function(touch) {
      pointers.set("touch_" + touch.identifier, {
        x: touch.clientX / canvas.width,
        y: touch.clientY / canvas.height
      });
    });
  });

  canvas.addEventListener("touchend", function(e) {
    e.preventDefault();
    forEach(e.changedTouches, function(touch) {
      pointers.delete("touch_" + touch.identifier);
    });
  });

  canvas.addEventListener("touchmove", function(e) {
    e.preventDefault();
    forEach(e.changedTouches, function(touch) {
      var pointer = pointers.get("touch_" + touch.identifier);
      pointer.x = touch.clientX / canvas.width;
      pointer.y = touch.clientY / canvas.height;
    });
  });

  draw();
  play();


  function play() {
    var audio = audioContext();

    updateAudio();

    function updateAudio() {
      var arr = [];
      pointers.forEach(function(pointer) {
        arr.push({
          frequency: x(pointer.x),
          volume: y(pointer.y),
          color: color
        });
      });

      ws.send(JSON.stringify(arr));
      audio.data(sineWaves);
      window.requestAnimationFrame(updateAudio);
    }
  }

  function draw() {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight - controls.clientHeight;
    var ctx = canvas.getContext('2d');

    ctx.fillStyle = "green";
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    updateCanvas();

    function updateCanvas() {
      ctx.fillStyle = "green";
      ctx.globalAlpha = 1;
      ctx.fillRect(0, 0, canvas.width, canvas.height);

      var hertz = 27.5;
      var innerHertz = hertz * twelve_root_2;
      do {
        while (innerHertz < 2 * hertz) {
          markLine("#33AA33", innerHertz);
          innerHertz *= twelve_root_2;
        }
        markLine("white", hertz);
        hertz *= 2;
      } while (hertz < +maxFrequency.value);

      sineWaves.forEach(function(d) {
        markTone(d);
      });

      window.setTimeout(function() {
       window.requestAnimationFrame(updateCanvas); 
      }, 60);
    }

    function markLine(color, frequency) {
      if (typeof frequency === "undefined") {
        return function(freq) { markLine(color, freq); };
      }
      var lineX = x_inv(frequency) * canvas.width;
      ctx.save();
      ctx.strokeStyle = color;
      ctx.globalAlpha = 1;
      ctx.beginPath();
      ctx.moveTo(lineX, 0);
      ctx.lineTo(lineX, canvas.height);
      ctx.stroke();
      return ctx.restore();
    };

    function markTone(sine) {
      var x = x_inv(sine.frequency);
      var y = y_inv(sine.volume);
      ctx.save();
      ctx.fillStyle = "white";
      ctx.fillStyle = sine.color;
      ctx.strokeStyle = "black";
      ctx.globalAlpha = 1;
      ctx.beginPath();
      ctx.arc(x * canvas.width, y * canvas.height, 15, 0, τ);
      ctx.stroke();
      ctx.fill();
      return ctx.restore();
    };
  }
}

function byId(id) { return document.getElementById(id); }

function forEach(iterable, fn) {
  return Array.prototype.forEach.call(iterable, fn);
}

function flatten(arrays) {
  return arrays.reduce(function(acc, arr) {
    if (Array.isArray(arr)) {
      arr.forEach(function(d) {
        acc.push(d);
      });
    } else {
      acc.push(arr);
    }
    return acc;
  }, []);
}
