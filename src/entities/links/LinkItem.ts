import { z } from 'zod';
import { createResource } from '../resource';

const resourceUrl = `/api/links/item`;

const linkItemResSchema = z.object({
  id: z.number(),
  title: z.string(),
  link: z.string(),
  orderNumber: z.number(),
  description: z.string(),
});

type LinkItemRes = z.infer<typeof linkItemResSchema>;
type NewLinkItem = Omit<LinkItemRes, 'id'> & {
  linkSectionId: number;
};

export const LinkItem = createResource<LinkItemRes, NewLinkItem>(resourceUrl, linkItemResSchema.parse);
