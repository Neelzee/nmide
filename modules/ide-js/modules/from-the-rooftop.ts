/**
 * Shout your mistake `from-the-rooftop`!
 *
 * This module is responsible for highlighting possible errors that occurs in a
 * module, that the module developer thinks is prudent to show the user. This
 * is typically done for IO operations, like if the user tries to open a file,
 * which no longer exists.
 */

import {
  cls,
  HtmlBuilder,
  isPrimAnd,
  UiBuilder,
  type Core,
  type Event,
  type Module
} from "@nmide/js-utils";
import { ERROR_ENDPOINT, NOTIFICATION_ID } from "../lib/constants";
import { dcError, type IDEError } from "../lib/errorUtils";

const name = "from-the-rooftop";

const NotificationModule: Module = {
  name,
  init: async (core: Core): Promise<void> => {
    await core.registerHandler(name, ERROR_ENDPOINT);
  },
  handler: async (evt: Event, core: Core): Promise<void> => {
    if (isPrimAnd(evt, ERROR_ENDPOINT)) {
      const { args } = evt.event;
      const error = dcError(args);
      if (!error) return;
      window.__nmideConfig__
        .log
        .error(`${name}: received error: ${JSON.stringify(error)}`);
      // NOTE: We are assuming that `NOTIFICATION_ID` exists
      await core.sendModification(
        new UiBuilder().add(buildError(error), NOTIFICATION_ID).build()
      );
    }
  }
}

const buildError = ({
  module,
  msg,
  error,
  triggeringEvent: event
}: IDEError) => new HtmlBuilder()
  .kind("li")
  .attrs(cls("ide-js-error"))
  .kids(
    new HtmlBuilder()
      .attrs(cls("ide-js-error-container"))
      .kids(
        new HtmlBuilder().attrs(cls("ide-js-error-module"))
          .kids(
            new HtmlBuilder()
              .kind("label")
              .attrs({ custom: ["for", "module"] }),
            new HtmlBuilder()
              .kind("input")
              .attrs(
                { custom: ["name", "module"] },
                { custom: ["value", module] },
                { custom: ["disabled", ""] },
              )
          ),
        new HtmlBuilder().attrs(cls("ide-js-error-msg"))
          .kids(
            new HtmlBuilder()
              .kind("label")
              .attrs({ custom: ["for", "msg"] }),
            new HtmlBuilder()
              .kind("input")
              .attrs(
                { custom: ["name", "msg"] },
                { custom: ["value", msg || ""] },
                { custom: ["disabled", ""] },
              )
          ),
        new HtmlBuilder().attrs(cls("ide-js-error-error"))
          .kids(
            new HtmlBuilder()
              .kind("label")
              .attrs({ custom: ["for", "error"] }),
            new HtmlBuilder()
              .kind("input")
              .attrs(
                { custom: ["name", "error"] },
                { custom: ["value", error || ""] },
                { custom: ["disabled", ""] },
              )
          ),
        new HtmlBuilder().attrs(cls("ide-js-error-event"))
          .kids(
            new HtmlBuilder()
              .kind("label")
              .attrs({ custom: ["for", "event"] }),
            new HtmlBuilder()
              .kind("input")
              .attrs(
                { custom: ["name", "event"] },
                { custom: ["value", event || ""] },
                { custom: ["disabled", ""] },
              )
          ),
      )
  );

export default NotificationModule;
