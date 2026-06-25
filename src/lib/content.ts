export type CollectionName = "writing" | "margins" | "projects";

export function entryUrl(collection: CollectionName, entry: { id: string }) {
  return `/${collection}/${entry.id}/`;
}

export function sortByDateDesc<T extends { data: { date: Date } }>(
  entries: T[],
) {
  return [...entries].sort(
    (a, b) => b.data.date.valueOf() - a.data.date.valueOf(),
  );
}

export function formatDate(date: Date) {
  return new Intl.DateTimeFormat("en-AU", {
    day: "numeric",
    month: "long",
    year: "numeric",
  }).format(date);
}

export function slugify(value: string) {
  return value
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/(^-|-$)/g, "");
}

