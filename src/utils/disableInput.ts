import * as htmx from 'htmx.org';

function disableInput(e: Event) {
  e.preventDefault();
  return false;
}

htmx.defineExtension('disable-input', {
  onEvent: function (name, evt) {
    let elt = evt.detail.elt;
    let target = elt.getAttribute('hx-disable-input');
    let targetInput =
      target == 'this' || !target ? [elt] : document.querySelectorAll(target);

    if (name === 'htmx:beforeRequest' && targetInput) {
      targetInput.forEach((el) => el.addEventListener('keydown', disableInput));
    } else if (name == 'htmx:afterRequest' && targetInput) {
      targetInput.forEach((el) => el.removeEventListener('keydown', disableInput));
    }
  },
});
