// TODO: Add docs

import { pipe } from "fp-ts/lib/function";
import { THtml } from "./THtml";
import { TState, TValue } from "./TMap";
import * as A from "fp-ts/Array";
import * as O from "fp-ts/Option";
import { make } from "fp-ts/Const";
import { Eq, fromEquals } from "fp-ts/Eq";
import { Eq as SEq } from "fp-ts/string";
import { Ins, ModIns, RemIns, UIns } from "./instruction";
import { sLookup } from "./Utils";

export type EventHandler = {
  handler: (c: Core, ...args: TValue[]) => Promise<unknown>;
  module: string;
};

export type EventHandlerUnknown = {
  handler: Function,
  module: string;
};

export type Event = {
  // Event name
  event: string,
  // Module id
  module: string,
};

export const EventEq: Eq<Event> = fromEquals(
  (
    { event: xe, module: xm },
    { event: ye, module: ym }
  ) => SEq.equals(xe, ye) && SEq.equals(xm, ym)
);

export type Core = {
  /**
   * UI Hierarchy
   */
  readonly ui: THtml;
  /**
   * State of the application
   */
  readonly state: TState;
  /**
   * List of events
   */
  readonly events: Event[];
  readonly eventThrower: (evt: Event) => void;
  readonly eventHandlers: Map<string, EventHandler[]>;
};

export type CoreModification = {
  readonly uiModifications: Ins<THtml>[];
  readonly stateModifications: Ins<TState>[];
  readonly eventModifications: Ins<Event>[];
  readonly newEventHandlers: [string, EventHandler][];
};

export type CoreModificationUnknown = {
  readonly uiModifications?: UIns[];
  readonly stateModifications?: UIns[];
  readonly eventModifications?: UIns[];
  readonly newEventHandlers?: [string, EventHandlerUnknown][];
};

export class CoreManager {
  private ui: THtml;
  private uiModifications: Ins<THtml>[] = [];

  private state: TState;
  private stateModifications: Ins<TState>[] = [];

  private events: Event[];
  private eventModifications: Ins<Event>[] = [];

  private newEventHandlers: [string, EventHandler][] = [];

  public constructor(core: Core) {
    this.ui = core.ui;
    this.state = core.state;
    this.events = core.events;
  }

  private traverseUI(ui: THtml, f: (ui: THtml) => boolean): O.Option<THtml> {
    return f(ui)
      ? O.some(ui)
      : pipe(
        ui.kids,
        A.findFirst(f),
      );
  }

  public findUI(f: (ui: THtml) => boolean): THtml | undefined {
    return pipe(
      this.traverseUI(this.ui, f),
      O.getOrElse<THtml | undefined>(() => undefined),
    );
  }

  public addUI(ui: THtml, f: (n: THtml) => boolean): CoreManager {
    const ins: ModIns<THtml, "kids"> = {
      f,
      field: "kids",
      g: (kids: THtml[]) => A.append(ui)(kids)
    };
    this.uiModifications.push(ins);
    return this;
  }

  public modifyUI<G extends keyof THtml = keyof THtml>(
    f: (n: THtml) => boolean,
    field: G,
    g: (field: THtml[G]) => THtml[G]
  ): CoreManager {
    const ins: ModIns<THtml, G> = {
      f,
      field,
      g,
    };
    this.uiModifications.push(ins);
    return this;
  }

  public removeUI(f: (n: THtml) => boolean): CoreManager {
    const ins: RemIns<THtml> = { f };
    this.uiModifications.push(ins);
    return this;
  }

  public findField<T extends TValue = TValue>(field: string): T | undefined {
    return pipe(
      this.state,
      sLookup<T>(field),
      O.match(
        () => undefined,
        make<T>,
      ),
    );
  }

  public build(): CoreModification {
    return {
      newEventHandlers: this.newEventHandlers,
      uiModifications: this.uiModifications,
      stateModifications: this.stateModifications,
      eventModifications: this.eventModifications,
    };
  }
};
