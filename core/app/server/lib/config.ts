import { serverRuntime } from "./app";

const Config = {
  runtimes:
  {
    handlers: [serverRuntime.handler],
    initializers: [serverRuntime.init]
  },
};

export default Config;