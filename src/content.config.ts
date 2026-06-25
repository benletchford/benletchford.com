import { defineCollection, z } from "astro:content";
import { glob } from "astro/loaders";

const writingFields = {
  title: z.string(),
  date: z.coerce.date(),
  section: z.string(),
  description: z.string(),
  featured: z.boolean().optional().default(false),
  draft: z.boolean().optional().default(false),
  tags: z.array(z.string()).optional().default([]),
};

const writing = defineCollection({
  loader: glob({
    pattern: "**/*.{md,mdx}",
    base: "./src/content/writing",
  }),
  schema: z.object(writingFields),
});

const margins = defineCollection({
  loader: glob({
    pattern: "**/*.{md,mdx}",
    base: "./src/content/margins",
  }),
  schema: z.object(writingFields),
});

const projects = defineCollection({
  loader: glob({
    pattern: "**/*.{md,mdx}",
    base: "./src/content/projects",
  }),
  schema: z.object({
    title: z.string(),
    date: z.coerce.date(),
    section: z.string(),
    description: z.string(),
    featured: z.boolean().optional().default(false),
    draft: z.boolean().optional().default(false),
    status: z.string().optional(),
    url: z.string().url().optional(),
    repository: z.string().url().optional(),
    tags: z.array(z.string()).optional().default([]),
  }),
});

export const collections = {
  writing,
  margins,
  projects,
};

