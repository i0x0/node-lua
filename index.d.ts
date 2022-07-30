class Lua {
  public instance: LuaInstance

  load(x: string): LuaResponse

  add(x: string, y: string): LuaResponse

  call(x: string): ExtendedLuaResponse
}

interface LuaResponse {
  err: null | string
}

type ExtendedLuaResponse = LuaResponse & {
  value: string | number | bool
}