/// TODO: Add docs

import { pipe } from "fp-ts/lib/function";
import * as A from "fp-ts/Array";
import * as O from "fp-ts/Option";

export type Ins<T extends object, G extends keyof T = keyof T>
  = RemIns<T>
  | ModIns<T, G>

// Delete
export type RemIns<T>
  = {
    /**
     * Delete the first `node` that fulfills the predicate
     */
    f: (t: T) => boolean
  };

// Modify
export type ModIns<T extends object, G extends keyof T = keyof T>
  = {
    /**
     * Apply `g` on field `G` on the first `node` that fulfills the predicate
     */
    f: (t: T) => boolean
    , field: G
    , g: (o: T[G]) => T[G]
  };

export type UIns
  = URemIns
  | UModIns

// Delete
export type URemIns
  = {
    /**
     * Delete the first `node` that fulfills the predicate
     */
    f: Function
  };

// Modify
export type UModIns
  = {
    /**
     * Apply `g` on field `G` on the first `node` that fulfills the predicate
     */
    f: Function
    , field: string
    , g: Function
  };

export const isModIns = <
  T extends object,
  G extends keyof T = keyof T,
>(
  i: Ins<T, G>
): i is ModIns<T, G> => "field" in i;


// NOTE: Wrapper type for _graphifying_ objects
// TODO: Add docs
export type Box<T extends object> = {
  box: T,
  getKids: (b: Box<T>) => Box<T>[],
  addKids: (box: Box<T>, ...kids: Box<T>[]) => Box<T>,
};

export const findInBox = <T extends object>(f: (t: T) => boolean) =>
  ({ box, getKids, ...xs }: Box<T>): O.Option<T> => f(box)
    ? O.some(box)
    : pipe(
      { box, getKids, ...xs },
      getKids,
      ys => pipe(
        ys,
        A.findFirst(({ box }) => f(box)),
        O.match<Box<T>, O.Option<T>>(
          () => pipe(
            ys,
            // NOTE: This is stupid
            A.filterMap(findInBox(f)),
            zs => A.isEmpty(zs)
              ? O.none
              : O.some(zs[0]),
          ),
          ({ box }) => O.some(box),
        ),
      )
    );
