import mdx from "@astrojs/mdx";
import { unified } from "@astrojs/markdown-remark";
import sitemap from "@astrojs/sitemap";
import { defineConfig } from "astro/config";

export default defineConfig({
  site: "https://benletchford.com",
  integrations: [mdx(), sitemap()],
  markdown: {
    processor: unified({
      gfm: true,
    }),
    shikiConfig: {
      theme: "github-light",
    },
  },
});
