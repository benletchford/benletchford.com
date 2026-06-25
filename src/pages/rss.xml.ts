import rss from "@astrojs/rss";
import { getCollection } from "astro:content";
import type { APIContext } from "astro";
import { entryUrl } from "@/lib/content";

export async function GET(context: APIContext) {
  const writing = await getCollection("writing", ({ data }) => data.draft !== true);
  const margins = await getCollection("margins", ({ data }) => data.draft !== true);

  const items = [
    ...writing.map((entry) => ({ collection: "writing" as const, entry })),
    ...margins.map((entry) => ({ collection: "margins" as const, entry })),
  ].sort((a, b) => b.entry.data.date.valueOf() - a.entry.data.date.valueOf());

  return rss({
    title: "Ben Letchford",
    description: "Essays, margins, and project notes from Ben Letchford.",
    site: context.site ?? "https://benletchford.com",
    items: items.map(({ collection, entry }) => ({
      title: entry.data.title,
      description: entry.data.description,
      pubDate: entry.data.date,
      link: entryUrl(collection, entry),
      categories: [entry.data.section, ...entry.data.tags],
    })),
    customData: "<language>en-au</language>",
  });
}

