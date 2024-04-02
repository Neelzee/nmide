export type FolderOrFile = Folder | File;

export type File = {
  name: string,
  extension: string,
  path: string
};

export type Folder = {
  name: string,
  path: string,
  content: FolderOrFile[]
};

export type NmideError<T> = {
  val: T,
  rep: NmideReport | null
};

export type NmideReport = {
  msg: string,
  lvl: ErrorLevel,
  tag: [],
  stack: [],
  origin: string
};

export enum ErrorLevel {
  Low,
  Medium,
  High,
  Unknown
};
