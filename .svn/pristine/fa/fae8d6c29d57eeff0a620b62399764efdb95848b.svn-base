// output/Data.Bounded/foreign.js
var topChar = String.fromCharCode(65535);
var bottomChar = String.fromCharCode(0);
var topNumber = Number.POSITIVE_INFINITY;
var bottomNumber = Number.NEGATIVE_INFINITY;

// output/Data.Maybe/index.js
var Nothing = /* @__PURE__ */ function() {
  function Nothing2() {
  }
  ;
  Nothing2.value = new Nothing2();
  return Nothing2;
}();
var Just = /* @__PURE__ */ function() {
  function Just2(value0) {
    this.value0 = value0;
  }
  ;
  Just2.create = function(value0) {
    return new Just2(value0);
  };
  return Just2;
}();
var maybe = function(v) {
  return function(v1) {
    return function(v2) {
      if (v2 instanceof Nothing) {
        return v;
      }
      ;
      if (v2 instanceof Just) {
        return v1(v2.value0);
      }
      ;
      throw new Error("Failed pattern match at Data.Maybe (line 237, column 1 - line 237, column 51): " + [v.constructor.name, v1.constructor.name, v2.constructor.name]);
    };
  };
};

// output/Data.Nullable/foreign.js
var nullImpl = null;
function notNull(x) {
  return x;
}

// output/Data.Nullable/index.js
var toNullable = /* @__PURE__ */ maybe(nullImpl)(notNull);

// output/Effect.InstallPlugin/foreign.js
var installPlugin = (name) => (plugin) => window.plugins.set(name, plugin);

// output/Main/index.js
var view = function(v) {
  return {
    kind: "P",
    attrs: [],
    kids: [],
    text: toNullable(new Just("Hello, World!"))
  };
};
var update = function(v) {
  return function(v1) {
    return [];
  };
};
var init = function(v) {
  return [];
};
var main = /* @__PURE__ */ installPlugin("PureScriptPlugin")({
  init,
  update,
  view
});
export {
  main
};
