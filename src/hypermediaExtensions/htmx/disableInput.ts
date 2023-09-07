import * as htmx from 'htmx.org';

function disableInput(e: Event) {
  e.preventDefault();
  return false;
}

const EVENTS = ['htmx:beforeRequest', 'htmx:beforeRequest'];

htmx.defineExtension('disable-input', {
  onEvent(name, evt) {
    let elt = evt.detail.elt;
    if (
      !EVENTS.includes(name) ||
      !(elt instanceof HTMLElement) ||
      'hxInputDisabled' in elt.dataset
    ) {
      return;
    }
    let target = elt.getAttribute('hx-disable-input');
    let targetInput =
      target === 'this' || !target ? [elt] : document.querySelectorAll(target);

    if (name === 'htmx:beforeRequest') {
      elt.dataset.hxInputDisabled = '';
      targetInput.forEach((el) => el.addEventListener('keydown', disableInput));
    } else if (name == 'htmx:afterRequest') {
      delete elt.dataset.hxInputDisabled;
      targetInput.forEach((el) => el.removeEventListener('keydown', disableInput));
    }
  },
});
