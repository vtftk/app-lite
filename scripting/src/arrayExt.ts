// Define array extensions
Array.prototype.random = function () {
  return this[Math.floor(Math.random() * this.length)];
};

declare global {
  // Array prototype extensions
  interface Array<T> {
    random(): T;
  }
}

export {};
