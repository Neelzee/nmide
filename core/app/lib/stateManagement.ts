import {
  GroupBy,
  TMap,
  TValue
} from "@nmide/js-utils";
import * as E from "fp-ts/Either";
import * as A from "fp-ts/Array";
import * as NA from "fp-ts/NonEmptyArray";
import * as O from "fp-ts/Option";
import * as S from "fp-ts/string";
import * as T from "fp-ts/Tuple";
import { pipe } from "fp-ts/lib/function";
import { fromEquals } from "fp-ts/Eq";

/**
 * Coaleces states
 *
 * Takes list of states from all plugins, checks for `collisions`. It returns a
 * list of `Either [(String, TMap)] ([(String, TMap)], String)`. If it is a
 * `collision`, then it's a `Right ([(String, TMap)], String)`, which is a tuple
 * where the first element is a list of tuples, being the plugin and their
 * state, and the last element being the field that the collision occurred on.
 * The other value: `Left [(String, TMap)]`, are the plugin state that has no
 * collision.
 *
 * `collision`: A collision between two states occurs if they share the same
 * field.
 *
 * Haskell example of the code
 * @example
 * ```haskell
 * stateHandler :: [(String, TMap)] -> [Either [(String, TMap)] ([(String, TMap)], String)]
 * stateHandler xs = map partitionStateCollision (groupBy stateCollision xs)
 *  where
 *    -- Returns true if they have the same fields
 *    stateCollision :: (String, TMap) -> (String, TMap) -> Bool
 *    stateCollision [] _ = false
 *    stateCollision ((a, _):xs) ys
 *      | a `elem` (map fst ys) = true
 *      | otherwise = stateCollision xs ys
 *    {- Returns the collision-field
 *       Is only called in the context where there is a collision.
 *    -}
 *    getCollisionField :: [(String, TMap)] -> String
 *    getCollisionField [] = "" -- Will never happen
 *    getCollisionField ((a, _):ys)
 *      | a `elem` (map fst xs) = a
 *      | otherwise = getCollisionField ys
 *    partitionStateCollision :: [[(String, TMap)]] -> [Either [(String, TMap)] ([(String, TMap)], String)]
 *    partitionStateCollision [] = []
 *    partitionStateCollision ([ys]:xs) = Left ys : partitionStateCollision xs
 *    partitionStateCollision (ys:xs) =  Right (ys, getCollisionField ys) : partitionStateCollision xs
 * ```
 *
 * There are several different ways to correct a collision between two
 * states:
 *
 *  1. If the states are of same type:
 *    1.1 If the value from one of the colliders are unchanged from the previous state:
 *      1.1.1 Keep the new value OR Keep the old value
 *    1.2 Else:
 *      1.2.1 Apply the types semigroup operator to the fields.
 *  2. Else:
 *    2.1 If the value from one of the colliders are unchanged from the previous state:
 *      2.1.1 Keep the new value OR Keep the old value
 *    2.2 Else:
 *      2.2.1 Keep the lhs value OR Keep the rhs value
 *
 *  Since the states are ordered by the name of the Plugin they come from, we
 *  have a consistent ordering of lhs and rhs, so if the same plugins give a
 *  `collision` on the same input, given that all plugins are pure, the resulting
 *  state will be the same everytime.
 *  The problem is that applying some function on the values could be an
 *  unwanted way to resolve `collision`s. So the standard way, will be to log
 *  the `collision`, and then drop both states. So even if two states has A and B
 *  amount of fields, and just 1 `collision`, we will drop A + B amount of fields.
 *  So, for a plugin developer, they should avoid `collisions`.
 */
export const stateHandler = (
  xs: [string, TMap][]
): E.Either<[[string, TMap][], string], [string, TMap]>[] => {
  // TODO: Add docs
  const stateCollision = (
    [p, a]: [string, TMap],
    [pb, b]: [string, TMap]
  ): boolean => {
    if (A.isNonEmpty(a)) {
      return pipe(
        NA.head(a),
        ([field, _]) => A.elem(S.Eq)(field)(A.map(T.fst)(b)),
        r => {
          if (!r) {
            return stateCollision([p, NA.tail(a)], [pb, b]);
          } else {
            return r;
          }
        },
        el => el,
      );
    } else {
      return false;
    }
  };

  // TODO: Add docs
  const getCollisionField = (ys: [string, TMap][]): string => pipe(
    ys,
    A.head,
    O.map<[string, TMap], TMap>(([_, m]) => m),
    O.match(
      () => O.none,
      A.head,
    ),
    O.map<[string, TValue], string>(T.fst),
    O.map<string, [string, boolean]>(a => [a, A.elem(S.Eq)(a)(
      pipe(
        A.map<[string, TMap], TMap>(T.snd)(xs),
        A.flatten,
        A.map<[string, TValue], string>(T.fst),
      ))]),
    O.match<[string, boolean], string>(
      // HACK: This will not happen, because `getCollisionField` is only
      // called if there is a collision. There is probably a better way to
      // handle the Option.none case.
      () => "",
      ([s, b]) => {
        if (b) {
          return s;
        } else {
          return getCollisionField(
            // HACK: I am unsure how to encode in the typesystem that this
            // function is total.
            O.getOrElse<[string, TMap][]>(() => [])(A.tail(xs))
          );
        }
      },
    ),
  );

  // TODO: Add docs
  const partitionStateCollision = (xs: [string, TMap][][]): E.Either<[[string, TMap][], string], [string, TMap]>[] => {
    return pipe(
      xs,
      A.map<[string, TMap][], E.Either<[[string, TMap][], string], [string, TMap]>>(ys => {
        if (ys.length === 1) {
          return E.right(ys[0]);
        } else {
          return E.left([ys, getCollisionField(ys)]);
        }
      }),
    );
  };
  return pipe(
    xs,
    GroupBy(fromEquals(stateCollision)),
    partitionStateCollision,
  );
};
