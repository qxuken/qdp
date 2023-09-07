import { isHTMLElement, isHTMLInputElement } from '../../utils';

const INPUT_EVENT = 'input';

export class SyncInputSize {
  static current?: SyncInputSize;

  static initOn(eventNames: Array<string>, el?: Element) {
    SyncInputSize.current?.cleanup();
    SyncInputSize.current = new SyncInputSize(eventNames, el);
  }

  constructor(
    private eventNames: Array<string>,
    private element: Element = document.body,
  ) {
    for (let event of eventNames) {
      element.addEventListener(event, this.subscribeElements.bind(this), {
        passive: true,
      });
    }
  }

  handleEvent(event: Event) {
    if (!isHTMLInputElement(event.target)) {
      return;
    }
    let minSize = Number(event.target.dataset.syncInputSizeMin) || 0;
    let modification = Number(event.target.dataset.syncInputSizeModification) || 0;
    event.target.size = Math.max(event.target.value.length + modification, minSize);
  }

  private isElementNeedsInitialization(element: Element): element is HTMLInputElement {
    return (
      isHTMLInputElement(element) &&
      'syncInputSize' in element.dataset &&
      !('syncInputSizeEnabled' in element.dataset)
    );
  }

  private getElements(target?: EventTarget | null): Array<HTMLInputElement> {
    let root: HTMLElement = (isHTMLElement(target) && target) || document.body;
    let elements = Array.from(
      root.querySelectorAll(
        'input[data-sync-input-size]:not([data-sync-input-size-enabled])',
      ),
    ).filter(this.isElementNeedsInitialization.bind(this));
    if (this.isElementNeedsInitialization(root)) {
      elements.push(root);
    }
    return elements;
  }

  subscribeElements(event: Event) {
    for (let element of this.getElements(event.target)) {
      element.dataset.syncInputSizeEnabled = '';
      element.addEventListener(INPUT_EVENT, this.handleEvent, {
        passive: true,
        capture: false,
      });
    }
  }

  cleanup() {
    for (let element of document.querySelectorAll(
      'input[data-sync-input-size-enabled]',
    )) {
      if (!isHTMLInputElement(element)) {
        continue;
      }
      element.removeEventListener(INPUT_EVENT, this.handleEvent);
      delete element.dataset.syncInputSizeEnabled;
    }
    for (let event of this.eventNames) {
      this.element.removeEventListener(event, this.subscribeElements);
    }
    delete SyncInputSize.current;
  }
}
