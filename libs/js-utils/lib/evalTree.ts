import * as A from "fp-ts/Array";
import { pipe } from "fp-ts/lib/function";
import { MonoidAny } from "fp-ts/boolean";
import { Ins, isModIns } from "./instruction";
import { Node, Tree } from "./tree";

// TODO: Add docs
export const evalTree = <T>(t: Tree<Node<T>>, ins: Ins<T>[]): Tree<Node<T>> => pipe(
  ins,
  A.reduce<Ins<T>, Tree<Node<T>>>(t, (accTree, i) =>
    isModIns(i)
      ? pipe(
        accTree.root,
        traverseAndApply(findById<T>(i.node.id))(i.f),
        root => { return { root } },
      )
      : pipe(
        accTree.root,
        traverseAndApply(findParent(i.node))(removeKid(i.node)),
        root => { return { root } },
      )
  )
);

const removeKid = <T>({ id }: Node<T>) => (parent: Node<T>): Node<T> => {
  return {
    ...parent,
    kids: pipe(
      parent.kids,
      A.filter(({ id: kid }) => kid !== id),
    ),
  };
};

// NOTE: ID must be unique
const findById = <T>(nid: string) => (n: Node<T>): boolean => nid === n.id;

// TODO: Add docs
// NOTE: `f` must be injective!
const traverseAndApply = <T>(f: ((n: Node<T>) => boolean)) =>
  (g: ((n: Node<T>) => Node<T>)) =>
    (n: Node<T>): Node<T> => f(n)
      ? g(n)
      : {
        ...n,
        kids: A.map(traverseAndApply(f)(g))(n.kids),
      }

// TODO: Add docs
const findParent = <T>({ id: kid }: Node<T>) =>
  (n: Node<T>): boolean => pipe(
    n.kids,
    A.foldMap(MonoidAny)(({ id }) => id === kid),
  );
