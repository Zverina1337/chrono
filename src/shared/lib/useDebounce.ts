export const useDebounce = <T extends (...args: any[]) => any>(callback: T, time: number) => {
  let timer: undefined | number = undefined;

  return function (...args: Parameters<T>) {
    clearTimeout(timer);
    timer = setTimeout(() => callback(...args), time);
  };
};
