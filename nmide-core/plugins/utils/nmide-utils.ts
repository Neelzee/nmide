/**
 * Gets the first element of a tuple
 */
const Fst = <K>([k, _]: [K, unknown]): K => k;


/**
 * Gets the second element of a tuple
 */
const Snd = <V>([_, v]: [unknown, V]): V => v;

/**
 * Helper function too see if a map has the given key
 */
const ContainsKey = (key: string): (el: [string, unknown]) => boolean =>
  (el: [string, unknown]): boolean => Fst(el) === key;

/**
 * Looks up the value, with the given key, returning the value,
 * or undefined if not found
 */
const Lookup = <T>(key: string, map: [string, T][]): T | undefined => {
  const val = map.find(ContainsKey(key));
  return val === undefined ? undefined : Snd(val);
};


/**
 * Looks up the value, with the given key, returning the value,
 * or the default value given
 */
const LookupOrDefault = <T>(val: T): (key: string, map: [string, T][]) => T =>
  (key: string, map: [string, T][]): T => {
    const res = Lookup(key, map);
    return res === undefined ? val : res;
  };

/**
 * Inserts the given value, with the supplied key, returning true if it
 * a field was overwritten
 */
const Insert = <T>(key: string, val: T, map: [string, T][]): boolean => {
  const exists = Lookup(key, map) !== val;
  if (exists) {
    const index = map.findIndex(ContainsKey(key));
    map[index] = [key, val];
  } else {
    map.push([key, val]);
  }
  return exists;
};

/**
 * Looks up the given key, and maps the applied function on it, returning true
 * if the operation took place.
 */
const LookupMap = <T>(func: (a: T) => T) =>
  (key: string, map: [string, T][]): boolean => {
    const val = Lookup(key, map);
    if (val === undefined) return false;
    const index = map.findIndex(ContainsKey(key));
    map[index] = [key, func(val)];
    return true;
  }

export {
  Lookup,
  LookupOrDefault,
  LookupMap,
  Insert,
};
