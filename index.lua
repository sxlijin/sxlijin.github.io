function Pandoc(doc)
  local posts_md = {}

  for _, postPath in ipairs(pandoc.system.list_directory("posts")) do
    local year, month, day = postPath:match("(%d+)-(%d+)-(%d+)-.*.md")
    local timestamp = os.time({year = year, month = month, day = day})
    local formattedDate = os.date("%b %Y", timestamp)

    local postPandoc =
      pandoc.read(io.open("posts/" .. postPath):read("*all"), "gfm")

    table.insert(
        posts_md,
        1,
        string.format("  * <div class=\"post-date\">%s</div> [%s](%s)",
                      formattedDate,
                      pandoc.utils.stringify(postPandoc.meta["title"]),
                      postPath:match("(.*).md")))
  end

  local blocks = pandoc.read(
    "<div class=\"post-list\">" .. table.concat(posts_md, "\n") .. "</div>",
    "markdown")

  for _, block in ipairs(blocks.blocks) do
    table.insert(doc.blocks, block)
  end

  return doc
end
