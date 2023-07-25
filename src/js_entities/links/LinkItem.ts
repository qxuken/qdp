import {  createResource } from '@entities/Resource';
import { z } from 'zod';

const resourceUrl = `/api/links/item`;

const linkItemResSchema = z.object({
  id: z.number(),
  title: z.string(),
  link: z.string(),
  orderNumber: z.number(),
  description: z.string(),
});

type LinkItemRes = z.infer<typeof linkItemResSchema>;
type NewLinkItem = Omit<LinkItemRes, 'id' | 'order_number'> & {
  linkSectionId: number;
};

export const LinkItem = createResource<LinkItemRes, NewLinkItem>(
  resourceUrl,
  linkItemResSchema.parse,
);
