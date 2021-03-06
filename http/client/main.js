var τ = 2 * Math.PI;
var twelve_root_2 = Math.pow(2, 1/12);
var message_cache = "";

var ws = new WebSocket("{{ws_url}}" + window.location.pathname);
ws.onopen = app;
ws.onclose = function() {
  sleep(500).then(promptRefresh);
}

function app() {
  "use strict";
  var canvas = byId("canvas");
  var controls = byId("controls");
  var header = byId("header");
  var minFrequency = controls.querySelector("input[name='min-frequency']");
  var maxFrequency = controls.querySelector("input[name='max-frequency']");

  window.addEventListener("resize", function() {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight -
      controls.clientHeight - header.clientHeight;
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
      x: (e.clientX - canvas.offsetLeft) / canvas.width,
      y: (e.clientY - canvas.offsetTop) / canvas.height
    });
  });

  canvas.addEventListener("mouseup", function() {
    pointers.delete("mouse_0");
  });

  canvas.addEventListener("mousemove", function(e) {
    var mouse = pointers.get("mouse_0");
    if (mouse) {
      mouse.x = (e.clientX - canvas.offsetLeft) / canvas.width;
      mouse.y = (e.clientY - canvas.offsetTop) / canvas.height;
    }
  });

  canvas.addEventListener("touchstart", function(e) {
    e.preventDefault();
    forEach(e.changedTouches, function(touch) {
      pointers.set("touch_" + touch.identifier, {
        x: (touch.clientX - canvas.offsetLeft) / canvas.width,
        y: (touch.clientY - canvas.offsetTop) / canvas.height
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
      pointer.x = (touch.clientX - canvas.offsetLeft) / canvas.width;
      pointer.y = (touch.clientY - canvas.offsetTop) / canvas.height;
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

      var message = JSON.stringify(arr);
      if (message !== message_cache) {
        // Only broadcast diffs.
        message_cache = message;
        ws.send(JSON.stringify(arr));
      }
      audio.data(sineWaves);
      window.requestAnimationFrame(updateAudio);
    }
  }

  function draw() {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight -
      controls.clientHeight - header.clientHeight;
    var ctx = canvas.getContext('2d');
    var rainbow = [
      "red", "orange", "yellow", "green", "blue", "indigo", "violet"
    ];
    ctx.fillStyle = background();
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    updateCanvas();

    function updateCanvas() {
      ctx.fillStyle = background();
      ctx.globalAlpha = 1;
      ctx.fillRect(0, 0, canvas.width, canvas.height);

      var hertz = 27.5;
      var innerHertz;
      while (hertz < +maxFrequency.value) {
        innerHertz = hertz;
        markLine(rainbow[0], innerHertz);    // A
        markLine(rainbow[0], innerHertz *= twelve_root_2);    // A♯
        markLine(rainbow[1], innerHertz *= twelve_root_2);    // B
        markLine(rainbow[2], innerHertz *= twelve_root_2);    // C
        markLine(rainbow[2], innerHertz *= twelve_root_2);    // C♯
        markLine(rainbow[3], innerHertz *= twelve_root_2);    // D
        markLine(rainbow[3], innerHertz *= twelve_root_2);    // D♯
        markLine(rainbow[4], innerHertz *= twelve_root_2);    // E
        markLine(rainbow[5], innerHertz *= twelve_root_2);    // F
        markLine(rainbow[5], innerHertz *= twelve_root_2);    // F♯
        markLine(rainbow[6], innerHertz *= twelve_root_2);    // G
        markLine(rainbow[6], innerHertz *= twelve_root_2);    // G♯

        hertz *= 2;
      }

      sineWaves.forEach(function(d) {
        markTone(d);
      });

      window.setTimeout(function() {
       window.requestAnimationFrame(updateCanvas); 
      }, 60);
    }

    function background() {
      var gradient = ctx.createRadialGradient(
        canvas.width / 2, 0, canvas.height,
        canvas.width / 2, 0, 0
      );

      gradient.addColorStop(0, "lightblue");
      gradient.addColorStop(0.21, "lightblue");
      rainbow.forEach(function(color, i) {
        gradient.addColorStop(0.22 + i * 0.01, color);
      });
      gradient.addColorStop(0.22 + rainbow.length * 0.01, "lightblue");
      gradient.addColorStop(1, "lightblue");

      return gradient;
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
      ctx.arc(
        minmax(0, 1, x) * canvas.width,
        y * canvas.height,
        15, 0, τ
      );
      ctx.stroke();
      ctx.fill();
      return ctx.restore();
    };
  }
}

function promptRefresh() {
  var modal = byId("prompt-refresh-modal");
  modal.style.display = "block";
}

byId("refresh-button").addEventListener("click", function(e) {
  e.preventDefault();
  window.location.reload();
})

function byId(id) { return document.getElementById(id); }

function sleep(ms) {
  return {
    then: function(fn) {
      window.setTimeout(fn, ms);
    }
  }
}

function forEach(iterable, fn) {
  return Array.prototype.forEach.call(iterable, fn);
}

function minmax(min, max, x) {
  return Math.max(min, Math.min(max, x));
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
