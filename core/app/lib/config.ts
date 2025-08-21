/**
 * Nmide frontend config.
 *
 * Currently the only thing that is _configured_ here, are the different
 * runtimes, which is just the JavaScript runtime.
 */

import runtime from "@nmide/js-core-std-lib";

const Config = {
  runtimes:
  {
    handlers: [runtime.handler],
    initializers: [runtime.init]
  },
};

export default Config;
