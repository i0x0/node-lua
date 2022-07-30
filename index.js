const lib = require("./index.node");

class Lua {
  constructor() {
    this.instance = lib.new();
  }

  load(x) {
    return lib.load(this.instance, x);
  }

  add(x, y) {
    return lib.value(this.instance, x, y);
  }

  call(x) {
    return lib.call(this.instance, x);
  }
}

module.exports = Lua;
module.exports.Lua = Lua;
module.exports.default = Lua;
