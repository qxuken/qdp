import { ajax, type AjaxResponse } from 'rxjs/ajax';
import type { Observable } from 'rxjs';
import { map } from 'rxjs';

const defaultHeaders = {
  'Content-Type': 'application/json',
} as const;

export const mapResponse = map(<R = unknown>(res: AjaxResponse<R>) => res.response);

export function postJSON<T, R = unknown>(
  url: string,
  body: T,
  headers?: Record<string, string>,
): Observable<R> {
  return ajax<R>({
    method: 'POST',
    url: url,
    headers: {
      ...defaultHeaders,
      ...headers,
    },
    body: body,
    responseType: 'json',
  }).pipe(mapResponse);
}

export function putJSON<T, R = unknown>(
  url: string,
  body: T,
  headers?: Record<string, string>,
): Observable<R> {
  return ajax<R>({
    method: 'PUT',
    url: url,
    headers: {
      ...defaultHeaders,
      ...headers,
    },
    body: body,
    responseType: 'json',
  }).pipe(mapResponse);
}

export function deleteJSON<R = unknown>(url: string, headers?: Record<string, string>): Observable<R> {
  return ajax<R>({
    method: 'DELETE',
    url: url,
    headers: {
      ...defaultHeaders,
      ...headers,
    },
    responseType: 'json',
  }).pipe(mapResponse);
}
