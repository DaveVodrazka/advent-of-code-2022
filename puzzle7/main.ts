import * as realFS from "fs";

class File {
  name: string;
  size: number;
  constructor(name: string, size: number) {
    this.name = name;
    this.size = size;
  }
}

class Folder {
  name: string;
  parent: Folder | null;
  subfolders: Folder[];
  files: File[];
  size?: number;
  constructor(name: string, parent: Folder | null) {
    this.name = name;
    this.parent = parent;
    this.subfolders = [];
    this.files = [];
  }
}

class FileSystem {
  root: Folder;
  current: Folder;
  partOne: number;
  partTwo: number;
  constructor() {
    this.root = new Folder("/", null);
    this.current = this.root;
    this.partOne = 0; // accumulate here sizes of folders of size <= 100000
    this.partTwo = 30000000; // size of the smallest folder that is big enough
  }
  public appendFile(name: string, size: number) {
    if (this.current.files.some((f) => f.name === name)) {
      console.log("Duplicate file");
      return;
    }
    this.current.files.push(new File(name, size));
  }
  public appendFolder(name: string) {
    if (this.current.subfolders.some((f) => f.name === name)) {
      console.log("Duplicate folder");
      return;
    }
    this.current.subfolders.push(new Folder(name, this.current));
  }
  public cd(path: string) {
    if (path === "/") {
      this.current = this.root;
      return;
    }
    if (path === "..") {
      this.up();
      return;
    }
    const sub = this.current.subfolders.find((f) => f.name === path);
    if (!sub) {
      throw Error(
        `Could not find ${path} in ${JSON.stringify(this.current.subfolders)}`
      );
    }
    this.current = sub;
  }
  private up() {
    if (!this.current.parent) {
      throw Error(`Cannot go up from root: ${JSON.stringify(this.current)}`);
    }
    this.current = this.current.parent;
  }
  private printRecursively(folder: Folder) {
    console.log(`folder: ${folder.name}, size: ${folder.size}`);
    folder.files.forEach(({ name, size }) =>
      console.log(`file: ${name}, size: ${size}`)
    );
    folder.subfolders.forEach((sub) => this.printRecursively(sub));
  }
  public print() {
    this.printRecursively(this.root);
  }
  private calculateSizeRecursively(folder: Folder): number {
    if (folder.size != undefined) {
      return folder.size;
    }
    let size = 0;
    folder.subfolders.forEach(
      (f) => (size += this.calculateSizeRecursively(f))
    );
    folder.files.forEach((f) => (size += f.size));
    folder.size = size;
    if (size <= 100000) {
      this.partOne += size;
    }
    return size;
  }
  public calculateSize() {
    this.calculateSizeRecursively(this.root);
  }
  public get fullSize() {
    if (this.root.size === undefined) {
      this.calculateSize();
    }
    return this.root.size as number;
  }
  public solvePartTwo() {
    const targetSize = 30000000 - (70000000 - this.fullSize);
    const findSize = (f: Folder) => {
      if (f.size === undefined) {
        return;
      }
      if (f.size >= targetSize && f.size < this.partTwo) {
        this.partTwo = f.size;
      }
      f.subfolders.forEach(findSize);
    };
    findSize(this.root);
  }
}

const fs = new FileSystem();
const lines = realFS
  .readFileSync("puzzle7/input.txt", { encoding: "utf-8" })
  .split("\n");

const handleCommand = (arr: string[]) => {
  if (arr[0] === "cd") {
    return fs.cd(arr[1]);
  }
};

const handleLine = (line: string): void => {
  const parts = line.split(" ");
  switch (parts[0]) {
    case "$":
      return handleCommand(parts.slice(1));
    case "dir":
      return fs.appendFolder(parts[1]);
    default:
      const size = parseInt(parts[0], 10);
      const name = parts[1];
      fs.appendFile(name, size);
  }
};

lines.forEach(handleLine);

fs.calculateSize();
console.log("Part one solution is:", fs.partOne);
fs.solvePartTwo();
console.log("Part two solution is:", fs.partTwo);
