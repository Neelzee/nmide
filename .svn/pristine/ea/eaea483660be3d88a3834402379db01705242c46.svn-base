import { type Core } from "./Core";
import { Event } from "./Event";
import { Html } from "./Html";
import { State } from "./State";

/**
 * A _valid_ Core variant, that simply logs the result of its invocations.
 * @returns Core
 */
export const DebugCore = (): Core => {
  return {
    ui: async (): Promise<Html> => {
      console.log("Core.UI");
      return { div: { kids: [], attrs: [], text: null } };
    },
    state: async (): Promise<State> => {
      console.log("Core.State");
      return {};
    },
    eventThrower: async (evt: Event): Promise<void> => {
      console.log("Core.eventThrower: ", evt);
    },
    registerHandler: async (
      name: string,
      event?: string,
      module?: string
    ): Promise<void> => {
      console.log("Core.registerHandler: ", name, event, module);
    }
  };
}