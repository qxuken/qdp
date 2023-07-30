import { map, type Observable } from 'rxjs';
import type { GenericResourceType, ValidateFn } from './Resource';
import { putJSON } from '../../utils/ajax';

export interface Orderable {
  orderNumber: number;
}

export type OrderExtension<T> = Orderable & {
  reorder(newOrderNumber: number): Observable<T>;
};
export function createOrderExtension<T extends Orderable & GenericResourceType<T>>(
  data: T,
  resourceUrl: string,
  validate: ValidateFn<T>,
): OrderExtension<T> {
  return {
    orderNumber: data.orderNumber,
    reorder(newOrderNumber) {
      return putJSON(`${resourceUrl}/${data.id}/reorder`, { newOrderNumber }).pipe(map(validate));
    },
  };
}
