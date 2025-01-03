import { Ins } from "./instruction";
import { MonoidAll } from "fp-ts/boolean";
import { foldMap } from "fp-ts/Array";
import { pipe } from "fp-ts/lib/function";

export default class TreeManager<T> {
  private instructions: Ins<T>[] = [];
  private tree: Tree<Node<T>>;
  constructor(tree: Tree<Node<T>>) {
    this.tree = tree;
  }

  private findNode(id: string, n: Node<T>): Node<T> | undefined {
    return id === n.id
      ? n
      : n.kids.map(k => this.findNode(id, k))[0]
  }

  // Eqviualent to a setter
  public modifyNode(id: string, f: ((old: Node<T>) => Node<T>)) {
    const node = this.findNode(id, this.tree.root);
    if (node === undefined) {
      // TODO: Do proper error handling
      return;
    }
    this.instructions.push({
      node,
      f
    });
  }

  public removeNode(id: string) {
    const node = this.findNode(id, this.tree.root);
    if (node === undefined) {
      // TODO: Do proper error handling
      return;
    }
    this.instructions.push({
      node,
    });
  }
}

export type Node<T> = T & {
  id: string,
  kids: Node<T>[];
};

export const toNode = <T extends object>(n: T): Node<T> => {
  /// TODO: Add some uuid generator to use here
  const id = "";
  if (
    "kids" in n
    && Array.isArray(n.kids) && n.kids.reduce((acc, x) => acc && related(x), true)) {
    return { id, ...n, kids: n.kids.map(toNode) };
  } else {
    return { id, kids: [], ...n };
  }
};

interface Nodeish {
  kids: Nodeish[]
}

const related = (x: unknown): x is Nodeish => typeof x === "object"
  && x !== null
  && "kids" in x
  && Array.isArray(x.kids)
  && pipe(
    x.kids,
    foldMap(MonoidAll)(related)
  );

export type Tree<T> = { root: Node<T> };

// TODO: Add docs
export const createTreeManager = <T extends object>(root: T): TreeManager<T> =>
  new TreeManager({ root: toNode<T>(root) });
