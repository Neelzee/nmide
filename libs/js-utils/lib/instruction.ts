/// TODO: Add docs

import * as E from "fp-ts/Either";
import * as A from "fp-ts/Array";
import { pipe } from "fp-ts/lib/function";
import { Eq } from "fp-ts/string";
import { Node } from "./tree";

export type Ins<T>
  = RemIns<T>
  | ModIns<T>

export type RemIns<T> = {
  node: Node<T>,
};

export type ModIns<T> = {
  node: Node<T>,
  f: (old: Node<T>) => Node<T>,
};

export const isModIns = <T>(n: Ins<T>): n is ModIns<T> =>
  "node" in n
  && "field" in n
  && "value" in n;


export type InsSet<T> = {
  instructions: Ins<T>[],
  module: string,
};

/**
 * Returns either, an Error if any `collision` occurred in the instruction sets,
 * or the combined instruction set, as if there is no collision, the order of
 * the `a` or `b` instruction set does not matter.
 *
 * **Collision**:
 * Occurs if:
 * - Two instructions modify the same Node, on the same field
 * - An instruction modify a Node which is related to another Node that has been
 *   removed.
 * _Modify_ in this context cover all instructions that affect a Node.
 */
export const collisionDetection = <T>(
  { instructions: aIns, module: a }: InsSet<T>,
  { instructions: bIns, module: b }: InsSet<T>,
  //
): E.Either<Error, Ins<T>[]> => pipe(
  aIns,
  A.map<Ins<T>, [Ins<T>, string]>(ia => [ia, a]),
  A.concat(A.map<Ins<T>, [Ins<T>, string]>(ib => [ib, b])(bIns)),
  A.flatMap<[Ins<T>, string], [Node<T>, string]>(([i, m]) => {
    if (isModIns(i)) {
      return [[i.node, m]];
    } else {
      return A.map<Node<T>, [Node<T>, string]>(n => [n, m])([...getKids(i.node)]);
    }
  }),
  A.map<[Node<T>, string], [string, string]>(([n, m]) => [n.id, m]),
  A.reduce<[string, string], E.Either<Error, string[]>>(E.right([]), (b, [n, _]) => pipe(
    b,
    E.match(
      E.left,
      arr => A.elem(Eq)(n)(arr)
        ? E.left(new Error("Collision"))
        : E.right([...arr, n])
    ),
  )),
  E.match(
    E.left,
    _ => E.right([...aIns, ...bIns])
  ),
);


const getKids = <T>({ kids }: Node<T>): Node<T>[] =>
  [...kids, ...A.flatMap(getKids)(kids)];
