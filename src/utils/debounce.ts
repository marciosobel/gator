export function debounce<T extends (...args: any[]) => any>(
  func: T,
  waitFor: number,
): (...args: Parameters<T>) => void {
  let timeoutId: number;
  return (...args: Parameters<T>): void => {
    if (typeof timeoutId !== "undefined") {
      clearTimeout(timeoutId);
    }

    timeoutId = setTimeout(() => {
      func(...args);
    }, waitFor);
  };
}
