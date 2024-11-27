import { App } from "@/app/App";

export const app = () => App();

export const debug = (a) => () => console.debug(a);

export const debugTMap = (a) => () => console.debug(a);

export const debugJson = (a) => () => console.debug(a);

export const debugEither = (a) => () => console.debug(a);
