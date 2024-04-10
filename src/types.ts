export type FolderOrFile = Folder | File;

export type File = {
  name: {
    Unix: [
      115,
      116,
      114,
      105,
      110,
      103
    ]
  },
  extension: {
    Unix: [
      115,
      116,
      114,
      105,
      110,
      103
    ]
  },
  path: {
    Unix: [
      115,
      116,
      114,
      105,
      110,
      103
    ]
  }
};

export type Folder = {
  name: {
    Unix: [
      115,
      116,
      114,
      105,
      110,
      103
    ]
  },
  path: {
    Unix: [
      115,
      116,
      114,
      105,
      110,
      103
    ]
  },
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