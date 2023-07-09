import type { Observable } from 'rxjs';
import { map } from 'rxjs';
import { deleteJSON, postJSON, putJSON } from '../ajax';

export type ValidateFn<T> = (validate: unknown) => T;
export type GenericResourceType<T> = { id: number } & Readonly<T>;
export type ResourceItemWithActions<T extends Object, U extends Object, E extends Object = {}> = T &
  E & {
    update$(data: U): Observable<ResourceItemWithActions<T, U, E>>;
    delete$(): Observable<null>;
  };
export type Resource<
  T extends GenericResourceType<O>,
  C extends Omit<T, 'id'> = Omit<T, 'id'>,
  U extends Partial<Omit<T, 'id'>> = Partial<Omit<T, 'id'>>,
  O extends Object = {},
  E extends Object = {},
> = {
  createItem(value: T): ResourceItemWithActions<T, U, E>;
  create$(data: C): Observable<ResourceItemWithActions<T, U, E>>;
};

export function createResource<
  T extends GenericResourceType<O>,
  C extends Omit<T, 'id'> = Omit<T, 'id'>,
  U extends Partial<Omit<T, 'id'>> = Partial<Omit<T, 'id'>>,
  O extends Object = {},
  E extends Object = {},
>(
  resourceUrl: string,
  validate: ValidateFn<T>,
  createItemExtension?: (data: T, resourceUrl: string, validate: ValidateFn<T>) => E,
): Resource<T, C, U, O, E> {
  function createResourceItem(value: T): ResourceItemWithActions<T, U, E> {
    let data = validate(value);
    let extensions = createItemExtension?.(data, resourceUrl, validate) ?? ({} as E);
    return {
      ...data,
      ...extensions,
      update$(updateData) {
        return putJSON<U, T>(`${resourceUrl}/${data.id}`, updateData).pipe(map(createResourceItem));
      },
      delete$() {
        return deleteJSON(`${resourceUrl}/${data.id}`).pipe(map(() => null));
      },
    };
  }
  return {
    createItem: createResourceItem,
    create$(data) {
      return postJSON<C, T>(resourceUrl, data).pipe(map(createResourceItem));
    },
  };
}
