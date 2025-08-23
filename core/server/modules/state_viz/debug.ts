import module from "./module";
import { debug_module } from "@nmide/js-debug-module";
import { renderer } from "../../core/app/lib/tsRenderer";
import type { Event } from "@nmide/js-utils";

const core = {
  eventThrower: async (evt: Event) => {
    window.debug_module.handler(evt)
      .catch(console.error);
  },
}

document.addEventListener("DOMContentLoaded", () => {
  window.state = {
    "user": {
      "obj": {
        "id": { "int": 123 },
        "name": { "str": "John Doe" },
        "isActive": { "bool": true },
        "scores": {
          "list": [
            { "float": 95.5 },
            { "float": 88.2 },
            { "float": 91.7 }
          ]
        },
        "preferences": {
          "obj": {
            "theme": { "str": "dark" },
            "notifications": { "bool": false },
            "language": { "str": "en-US" }
          }
        },
        "history": {
          "list": [
            {
              "obj": {
                "date": { "str": "2023-01-15" },
                "action": { "str": "login" }
              }
            },
            {
              "obj": {
                "date": { "str": "2023-01-16" },
                "action": { "str": "purchase" },
                "items": {
                  "list": [
                    { "str": "item1" },
                    { "str": "item2" }
                  ]
                }
              }
            }
          ]
        }
      }
    },
    "system": {
      "obj": {
        "status": { "str": "running" },
        "load": { "float": 0.75 },
        "components": {
          "list": [
            { "str": "database" },
            { "str": "cache" },
            { "str": "api" }
          ]
        },
        "config": {
          "obj": {
            "timeout": { "int": 30 },
            "retries": { "int": 3 },
            "fallback": { "bool": true }
          }
        }
      }
    },
    "nullValue": "null",
    "mixedList": {
      "list": [
        { "int": 1 },
        { "float": 2.5 },
        { "bool": true },
        { "str": "text" },
        "null",
        {
          "obj": {
            "key": { "str": "value" }
          }
        },
        {
          "list": [
            { "int": 10 },
            { "float": 20.5 }
          ]
        }
      ]
    },
    "deeplyNested": {
      "obj": {
        "level1": {
          "obj": {
            "level2": {
              "obj": {
                "level3": {
                  "obj": {
                    "level4": {
                      "obj": {
                        "value": { "str": "bottom" }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  };
  debug_module(module, core, renderer(core.eventThrower));
})