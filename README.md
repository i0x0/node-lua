# `node-lua`
> something probably only roblox devs would use

<p align="center">
  <a href="https://skillicons.dev">
    <img src="https://skillicons.dev/icons?i=js,ts,rust,lua"/>
  </a>
</p>

## Docs

### To install (do i feel like pushing to npm? mmmm no?)
```bash
npm install git+https://github.com/i0x0/node-lua.git
```
### Everything else

```ts
import Lua from "node-lua"
import assert from "assert"
// make a new instance of Lua
let lua = new Lua()

//probably a recommended function for simple logging lua errors
const luaError = (err: string, x: () => {}) => console.error("lua error:" + err)

//go-lang way of error handling
let { err } = lua.load("function hi() return "hello" end")

// if nothing logs then it worked perfectly
if (err) {
  luaError(err)
}

// call the same function we made
let { err, value } = lua.call("hi()")
if (err) {
  luaError(err)
} else {
  assert.equal(value, "hello")
}

// add a variable from ts to lua (only supports strings for now)
let { err } = lua.add("hello", "world")
if (err) {
  luaError(err)
}

// getting the variable we added earlier
let { err, value } = lua.call("hello")
if (err) {
  luaError(err)
} else {
  assert.equal(value, "world")
}

```