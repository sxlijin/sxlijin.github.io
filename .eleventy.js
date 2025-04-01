import sass from "sass";
import path from "path";
import { fileURLToPath } from "url";
import MarkdownIt from "markdown-it";
import mdPrism from "markdown-it-prism";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const getPosts = (collectionApi) => collectionApi.getFilteredByGlob("posts/*.md").map(post => {
  post.data.layout = "Post.njk";
  post.data.x_formatted_date = post.date.toISOString().split('T')[0];
  return post;
});

export default async (eleventyConfig) => {

  // Copy static assets
  eleventyConfig.addPassthroughCopy("assets");
  eleventyConfig.addPassthroughCopy("root-assets");

  // SCSS processing
  eleventyConfig.addTemplateFormats("scss");
  eleventyConfig.addExtension("scss", {
    outputFileExtension: "css",
    compile: async (str, inputPath) => {
      const result = sass.compile(inputPath, {
        loadPaths: [path.dirname(inputPath)],
      });
      return async () => result.css;
    },
  });

  // Collections
  eleventyConfig.addCollection("posts", getPosts);

  eleventyConfig.addCollection("pages", (collectionApi) => {
    return collectionApi.getFilteredByGlob("pages/*.md").map(page => {
      // console.log("page object", { pageTemplate: page.template._config })
      page.outputPath = page.outputPath.replace('_site/pages/', '_site/');
      page.data.layout = "Base.njk"

      if (page.inputPath === "./pages/index.md") {
        const posts = getPosts(collectionApi);

        // Sort in order of most to least recent
        // TODO: use lodash _.sortBy to define a sorting key instead
        posts.sort((post1, post2) => {
          return post2.data.x_formatted_date.localeCompare(post1.data.x_formatted_date)
        });

        page.rawInput += 
          posts.map(({ data: { x_formatted_date }, fileSlug, url }) => { 
            return `  * ${x_formatted_date}: [${fileSlug}](${url})`;
          }).join("\n");

        return page;
      }
    });
  });

  // Markdown configuration
  const md = new MarkdownIt({
    html: true,
    linkify: true,
    typographer: true,
  }).use(mdPrism);

  eleventyConfig.setLibrary("md", {
    render: (str) => md.render(str)
  });

  // Add date filter
  eleventyConfig.addFilter("formatDate", (date) => new Date(date).toLocaleDateString("en-US", {
      year: "numeric",
      month: "short",
      day: "numeric",
  }));

  // Add basepath filter
  eleventyConfig.addFilter("getBasepath", (path) => path.replace(/\.md$/, ""));

  return {
    dir: {
      input: ".",
      output: "_site",
      includes: "_includes",
      layouts: "_layouts",
    },
    templateFormats: ["md", "njk"],
    markdownTemplateEngine: "njk",
    htmlTemplateEngine: "njk",
    dataTemplateEngine: "njk",
  };
} 