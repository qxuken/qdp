export function isHTMLInputElement(element: unknown): element is HTMLInputElement {
  return element instanceof HTMLInputElement;
}
