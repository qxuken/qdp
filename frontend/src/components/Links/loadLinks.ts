import { fromEvent, tap, map } from 'rxjs';
import { z } from 'zod';

let linkItemResSchema = z.object({
  id: z.number(),
  title: z.string(),
  link: z.string(),
  order_number: z.number(),
  description: z.string(),
});

let linkSectionResSchema = z.object({
  id: z.number(),
  title: z.string(),
  order_number: z.number(),
  items: z.array(linkItemResSchema),
});

function renderSection(title: string) {}

export function loadLinks(id: string) {
  return fromEvent(document, 'DOMContentLoaded')
    .pipe(
      map(() => document.querySelector(id)),
      tap((linksEl) => {
        let sectionEl = document.createElement('link-section');
        // @ts-ignore
        sectionEl.hello = 'text';
        linksEl?.appendChild(sectionEl);
      }),
    )
    .subscribe((data) => console.log('emission', data));
}
