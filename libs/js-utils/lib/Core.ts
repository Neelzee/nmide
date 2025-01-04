// TODO: Add docs

import { pipe } from "fp-ts/lib/function";
import { THtml } from "./THtml";
import { TMap, TValue } from "./TMap";
import { Ins } from "./instruction";
import { tObj } from "./Types";
import TreeManager, { toNode, Node } from "./tree";
import * as A from "fp-ts/Array";
import * as O from "fp-ts/Option";

export type EventHandler = (c: Core, ...args: TValue[]) => Promise<unknown>;

export type Event = {
  // Event name
  event: string,
  // Module id
  module: string,
}

export type Core = {
  /**
   * UI Hierarchy
   */
  readonly ui: THtml;
  readonly uiModifications: Ins<THtml>[];
  /**
   * State of the application
   */
  readonly state: TMap;
  readonly stateModifications: Ins<TValue>[];
  /**
   * List of events
   */
  readonly events: Event[],
  readonly eventModifications: Ins<Event>[];
};

export class CoreManager {
  private core: Core;

  private ui: TreeManager<THtml>;

  // HACK: Since the state is just a list of tuples, with fields and their
  // corresponding value, it is like a tree, where the _root_ is ignored, and
  // all values are the children of the root. This is done to allow for a
  // unified collision solver to be used/available
  private state: TreeManager<TValue>;

  // HACK: Similar to the state, events is just a list of Events. A collision
  // here is not as likely, but still possible, so an empty event is used as the
  // root to allow for a unified collision solver
  private events: TreeManager<Event>;

  public constructor(core: Core) {
    this.core = core;
    this.ui = new TreeManager({ root: toNode(core.ui) });
    const id = "root";
    this.state = new TreeManager({
      root: {
        id,
        kids: pipe(
          core.state,
          A.map(([field, value]) => {
            return { id: field, kids: [] as Node<TValue>[], ...value };
          })
        ),
        ...tObj([])
      }
    });
    this.events = new TreeManager({
      root: {
        id,
        kids: pipe(
          core.events,
          A.map(toNode)
        ),
        event: "",
        module: ""
      }
    });
  }

  private traverseTree<T>(
    root: Node<T>,
    f: (n: Node<T>) => boolean
  ): O.Option<Node<T>> {
    const g = (n: Node<T>): n is Node<T> => f(n);
    return f(root) ? O.some(root) : pipe(
      O.tap<Node<T>, Node<T>>(
        A.findFirst<Node<T>, Node<T>>(g)(root.kids),
        p => A.findFirst<Node<T>, Node<T>>(g)(p.kids)
      ),
    )
  }

  private fromNode(n: Node<THtml>): THtml {
    const { id: _id, ...t } = n;
    return { ...t };
  };

  public findUI(f: (ui: THtml) => boolean): THtml | undefined {
    return pipe(
      this.traverseTree<THtml>(this.ui.tree.root, f),
      O.match(
        () => undefined,
        n => this.fromNode(n),
      ),
    );
  }

  public addUI(ui: THtml, f: (n: THtml) => boolean): CoreManager {
    return pipe(
      this.traverseTree<THtml>(this.ui.tree.root, f),
      O.map(({ id }) => this.ui.modifyNode(id, ({ kids, ...rem }) => {
        return { ...rem, kids: A.append(toNode(ui))(kids) };
      })),
      _ => this,
    );
  }

  public modifyUI(f: (n: THtml) => boolean, g: (n: THtml) => THtml): CoreManager {
    return pipe(
      this.traverseTree<THtml>(this.ui.tree.root, f),
      O.map(({ id }) => this.ui.modifyNode(id, old => {
        const { id: kid, ...node } = old;
        return { ...toNode(g(node)), id: kid };
      })),
      _ => this,
    );
  }

  public removeUI(f: (n: THtml) => boolean): CoreManager {
    return pipe(
      this.traverseTree<THtml>(this.ui.tree.root, f),
      O.map(({ id }) => this.ui.removeNode(id)),
      _ => this,
    );
  }

  public findField(field: string): TValue | undefined {
    return pipe(
      this.traverseTree<TValue>(this.state.tree.root, ({ id }) => id === field),
      O.match(
        () => undefined,
        ({ id: _id, kids: _kid, ...rem }) => {
          return { ...rem };
        },
      ),
    );
  }

  public addField(field: string, value: TValue): CoreManager {
    return pipe(
      this.traverseTree<TValue>(
        this.state.tree.root, ({ id }) => id === "root"
      ),
      O.map(({ id }) => this.state.modifyNode(id, ({ kids, ...rem }) => {
        return {
          ...rem,
          kids: A.append({
            id: field,
            kids: [] as Node<TValue>[],
            ...value
          })(kids)
        };
      })),
      _ => this,
    );
  }

  public modifyField(f: (n: TValue) => boolean, g: (n: TValue) => TValue): CoreManager {
    return pipe(
      this.traverseTree<TValue>(this.state.tree.root, f),
      O.map(({ id }) => this.state.modifyNode(id, old => {
        const { id: kid, ...node } = old;
        return { ...toNode(g(node)), id: kid };
      })),
      _ => this,
    );
  }

  public removeField(field: string): CoreManager {
    return pipe(
      this.traverseTree<TValue>(this.state.tree.root, ({ id }) => id === field),
      O.map(({ id }) => this.ui.removeNode(id)),
      _ => this,
    );
  }

  public addEvent(event: Event): CoreManager {
    return pipe(
      this.traverseTree<Event>(
        this.events.tree.root, ({ id }) => id === "root"
      ),
      O.map(({ id }) => this.events.modifyNode(id, ({ kids, ...rem }) => {
        return {
          ...rem,
          kids: A.append({
            id: event.event,
            kids: [] as Node<Event>[],
            ...event,
          })(kids)
        };
      })),
      _ => this,
    );
  }

  public modifyEvent(
    f: (n: Event) => boolean,
    g: (n: Event) => Event
  ): CoreManager {
    return pipe(
      this.traverseTree<Event>(this.events.tree.root, f),
      O.map(
        ({ id }) => this.events.modifyNode(
          id,
          ({ id: kid, kids, ...node }) => {
            return { ...g({ ...node }), id: kid, kids };
          })
      ),
      _ => this,
    );
  }

  public removeEvent(field: string): CoreManager {
    return pipe(
      this.traverseTree<Event>(this.events.tree.root, ({ id }) => id === field),
      O.map(({ id }) => this.ui.removeNode(id)),
      _ => this,
    );
  }

  public build(): Core {
    return {
      ...this.core,
      uiModifications: this.ui.modifications(),
      stateModifications: this.state.modifications(),
      eventModifications: this.events.modifications(),
    };
  }
};
